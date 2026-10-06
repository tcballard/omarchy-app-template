use std::{env, path::PathBuf, process::Command};

fn output(program: &str, args: &[&str]) -> String {
    let result = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("Cannot run {program}: {error}"));
    assert!(
        result.status.success(),
        "{program} failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).expect("UTF-8 tool output")
}

fn main() {
    println!("cargo:rerun-if-changed=ui/window.cpp");
    if env::var_os("CARGO_FEATURE_GUI").is_none() {
        return;
    }
    assert_eq!(
        env::var("HOST").unwrap(),
        env::var("TARGET").unwrap(),
        "Native builds only; configure a cross-toolchain before cross-compiling"
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let object = out.join("window.o");
    let flags = output("pkg-config", &["--cflags", "Qt6Widgets"]);
    let status = Command::new("c++")
        .args([
            "-std=c++17",
            "-fPIC",
            "-Wall",
            "-Wextra",
            "-Werror",
            "-c",
            "ui/window.cpp",
            "-o",
        ])
        .arg(&object)
        .args(flags.split_whitespace())
        .status()
        .expect("Install a C++ compiler and Qt 6 development files");
    assert!(status.success(), "Qt bridge compilation failed");
    let status = Command::new("ar")
        .arg("crs")
        .arg(out.join("libwindow.a"))
        .arg(object)
        .status()
        .expect("Install binutils");
    assert!(status.success(), "Qt bridge archive failed");
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=window");
    for flag in output("pkg-config", &["--libs", "Qt6Widgets"]).split_whitespace() {
        if let Some(lib) = flag.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={lib}");
        } else if let Some(dir) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={dir}");
        } else {
            println!("cargo:rustc-link-arg={flag}");
        }
    }
    println!("cargo:rustc-link-lib=stdc++");
}
