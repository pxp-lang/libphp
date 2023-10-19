use std::{path::PathBuf, env, ffi::OsStr, fmt::Debug, process::Command};

use bindgen::Builder;
use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=src/php.c");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let php_config = env::var("PHP_CONFIG").unwrap_or_else(|_| "php-config".to_string());

    let includes = exec(&[php_config.as_str(), "--includes"]);
    let includes = includes.split(' ').collect::<Vec<&str>>();

    let mut build = Build::new();
    for include in includes.iter() {
        build.flag(include);
    }
    build.file("src/php.c").compile("php-sys");

    let include_dirs = includes.iter().map(|include| &include[2..]).collect::<Vec<&str>>();

    for dir in include_dirs.iter() {
        println!("cargo:include={dir}");
    }

    let mut builder = Builder::default()
        .header("src/php.c")
        .allowlist_file("src/php.c")
        .clang_args(&includes)
        .derive_default(true);

    for dir in include_dirs.iter() {
        let path = PathBuf::from(dir).join(".*.h");
        builder = builder.allowlist_file(path.display().to_string());
    }

    let bindings_path = out_path.join("php_sys_bindings.rs");

    builder
        .generate()
        .expect("Failed to generate bindings.")
        .write_to_file(bindings_path)
        .expect("Failed to write bindings to file.");
}

fn exec<S: AsRef<OsStr> + Debug>(args: &[S]) -> String {
    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    let output = command.output().unwrap_or_else(|_| panic!("Failed to execute command {:?}", &args)).stdout;

    String::from_utf8(output).unwrap().trim().to_string()
}