#![allow(unused_variables)]

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use bindgen::Builder;

const PHP_VERSION: &str = "8.2";

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rerun-if-env-change=PHP_VERSION");

    if !target_exists("spc") {
        run_command_or_fail(
            target_dir(""),
            "git",
            &[
                "clone",
                "https://github.com/crazywhalecc/static-php-cli.git",
                "spc",
                "--depth=1",
            ],
        );
        run_command_or_fail(
            target_dir("spc"),
            "composer",
            &["update", "--no-dev", "-n", "--no-plugins"],
        );
        run_command_or_fail(
            target_dir("spc"),
            "php",
            &[
                "bin/spc",
                "download",
                "php-src,pkg-config,micro",
                format!("--with-php={}", PHP_VERSION).as_str(),
            ],
        );
        run_command_or_fail(
            target_dir("spc"),
            "php",
            &["bin/spc", "doctor", "--auto-fix"],
        );
        run_command_or_fail(
            target_dir("spc"),
            "php",
            &[
                "bin/spc",
                "build",
                "opcache",
                "--build-embed",
                "--enable-zts",
            ],
        );
    }

    let include_dir = target_dir("spc/buildroot/include/php");
    let lib_dir = target_dir("spc/buildroot/lib");

    println!("cargo:rustc-link-lib=static=php");
    println!("cargo:rustc-link-search=native={}", lib_dir);

    let includes = ["/", "Zend", "/main", "/TSRM"]
        .iter()
        .map(|folder| format!("-I{}/{}", &include_dir, &folder))
        .collect::<Vec<String>>();

    let bindings = Builder::default()
        .clang_args(&includes)
        .derive_default(true)
        .allowlist_type("zval")
        .allowlist_type("zend_constant")
        .allowlist_type("zend_fcall_info")
        .allowlist_function("zend_string_init")
        .allowlist_function("zend_call_function")
        .allowlist_function("_zend_new_array")
        .allowlist_function("zend_array_count")
        .allowlist_function("zend_hash_get_current_key_type_ex")
        .allowlist_function("zend_hash_get_current_key_zval_ex")
        .allowlist_function("zend_hash_get_current_data_ex")
        .allowlist_function("zend_hash_move_forward_ex")
        .allowlist_function("zend_eval_string_ex")
        .allowlist_function("php_embed_init")
        .allowlist_function("php_embed_shutdown")
        .allowlist_function("zend_compile_string")
        .allowlist_function("zend_get_type")
        .allowlist_function("zval_ptr_dtor")
        .allowlist_function("zend_stream_init_filename")
        .allowlist_function("php_execute_script")
        .allowlist_function("php_execute_simple_script")
        .allowlist_function("php_register_variable_ex")
        .allowlist_type("zend_function_entry")
        .allowlist_function("zend_register_functions")
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/wrapper.c")
        .includes(
            &includes
                .iter()
                .map(|s| s.as_str()[2..].to_string())
                .collect::<Vec<String>>(),
        )
        .flag("-fPIC")
        .flag("-m64")
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
    let fmt_cmd = format!("{} {}", cmd, args.join(" "));
    println!("Running command: \"{}\" in dir: {}", &fmt_cmd, dir);
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {} [cmd] {}", c, &fmt_cmd),
        Ok((false, None)) => panic!("Command got killed [cmd] {}", &fmt_cmd),
        Err(e) => panic!("Command failed with error: {} [cmd] {}", e, &fmt_cmd),
    }
}
