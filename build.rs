#![allow(unused_variables)]

use std::{env, fs, path::{Path, PathBuf}, process::Command};

use bindgen::Builder;

const PHP_VERSION_BRANCH: &'static str = "PHP-8.2.11";

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.c");

    let cpus = num_cpus::get();

    #[cfg(all(target_os = "linux"))]
    let default_link_static = false;
    #[cfg(all(target_os = "macos"))]
    let default_link_static = true;

    let php_version_branch = option_env!("PHP_VERSION_BRANCH").unwrap_or(PHP_VERSION_BRANCH);

    fs::create_dir_all(target_dir("")).expect("Failed to create target directory.");

    println!("cargo:rerun-if-env-change=PHP_VERSION_BRANCH");

    if ! target_exists("php-src/LICENSE") {
        println!("Setting up PHP (branch: {})", php_version_branch);

        run_command_or_fail(
            target_dir(""),
            "git",
            &[
                "clone",
                "https://github.com/php/php-src",
                format!("--branch={}", php_version_branch).as_str(),
                "--depth=1",
            ],
        );

        run_command_or_fail(target_dir("php-src"), "./scripts/dev/genfiles", &[]);
        run_command_or_fail(target_dir("php-src"), "./buildconf", &["--force"]);

        #[cfg(all(target_os = "linux"))]
        let config = &[
            "--enable-debug",
            "--enable-embed=shared",
            "--disable-cli",
            "--disable-cgi",
            "--enable-zts",
            // "--without-iconv",
            //"--disable-libxml",
            //"--disable-dom",
            //"--disable-xml",
            //"--disable-simplexml",
            //"--disable-xmlwriter",
            //"--disable-xmlreader",
            // "--without-pear",
            // "--with-libdir=lib64",
            // "--with-pic",
        ];
        #[cfg(all(target_os = "macos"))]
        let config = &[
            "--enable-debug",
            "--enable-embed=static",
            "--disable-cli",
            "--disable-cgi",
            "--enable-zts",
            "--disable-all",
        ];

        run_command_or_fail(target_dir("php-src"), "./configure", config);
        run_command_or_fail(target_dir("php-src"), "make", &["-j", cpus.to_string().as_str()]);
    }

    let include_dir = target_dir("php-src");
    let lib_dir = target_dir("php-src/libs");
    let link_type = "=static";

    println!("cargo:rustc-link-lib{}=php", link_type);
    println!("cargo:rustc-link-search=native={}", lib_dir);

    let includes = ["/", "/TSRM", "/Zend", "/main"]
        .iter()
        .map(|d| format!("-I{}{}", include_dir, d))
        .collect::<Vec<String>>();

    let bindings = Builder::default()
        .clang_args(includes)
        .derive_default(true)
        .allowlist_type("zval")
        .allowlist_function("zend_eval_string_ex")
        .allowlist_function("php_embed_init")
        .allowlist_function("php_embed_shutdown")
        .allowlist_function("zend_compile_string")
        .allowlist_function("zend_get_type")
        .allowlist_function("zval_ptr_dtor")
        .allowlist_function("zend_stream_init_filename")
        .allowlist_function("php_execute_script")
        .allowlist_function("php_execute_simple_script")
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/wrapper.c")
        .include(&include_dir)
        .flag("-fPIC")
        .flag("-m64")
        .include(&format!("{}/TSRM", include_dir))
        .include(&format!("{}/Zend", include_dir))
        .include(&format!("{}/main", include_dir))
        .static_flag(true)
        .compile("wrapper");
}

fn target_dir(path: &str) -> String {
    let out_dir = env::var("OUT_DIR").unwrap();
    format!("{}/{}", out_dir, path)
}

fn target_exists(path: &str) -> bool {
    Path::new(target_dir(path).as_str()).exists()
}

fn run_command_or_fail(dir: String, cmd: &str, args: &[&str]) {
    println!(
        "Running command: \"{} {}\" in dir: {}",
        cmd,
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => return,
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}