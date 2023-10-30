use std::{
    env,
    process::Command,
    path::Path,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if Path::new(&format!("{}/src/tests/akane/test.akane", cargo_manifest_dir)).exists() {
        akaneclib::compile(
            &format!("{}/src/tests/akane/test.akane", cargo_manifest_dir),
            &format!("{}/test.ll", out_dir)
        ).unwrap();
    }
    else {
        panic!("test.akane not exists.");
    }
    if Path::new(&format!("{}/test.ll", out_dir)).exists() {
        Command::new("llc-15")
        .args([
            "--filetype=obj",
            &format!("-o={}/test.o", out_dir),
            &format!("{}/test.ll", out_dir),
        ])
        .status()
        .or_else(|_|
            Command::new("llc")
            .args([
                "--filetype=obj",
                &format!("-o={}/test.o", out_dir),
                &format!("{}/test.ll", out_dir),
            ])
            .status()
        ).unwrap();
    }
    else {
        panic!("test.ll not exists.");
    }
    if Path::new(&format!("{}/test.o", out_dir)).exists() {
        Command::new("ar")
        .args([
            "r",
            &format!("{}/libakanectest.a", out_dir),
            &format!("{}/test.o", out_dir),
        ])
        .status()
        .unwrap();
    }
    else {
        panic!("test.o not exists.");
    }
    if !Path::new(&format!("{}/libakanectest.a", out_dir)).exists() {
        panic!("libakanectest.a not exists.");
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=akanectest");
    println!("cargo:rerun-if-changed=./src/tests/akane/test.akane");
}
