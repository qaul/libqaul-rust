extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let mut dst = Config::new("qaul.net").define("GUI", "CLI").build();

    dst.push("build");
    dst.push("src");
    dst.push("libqaul");

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=libqaul");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=libqaul");

    let version = env!("CARGO_MANIFEST_DIR");
    println!("###################### CURRENT DIRECTORY: {}", version);

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", version))
        .header("qaul/qaul.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let path = PathBuf::from("src/bindings/");
    bindings
        .write_to_file(path.join("qaul.rs"))
        .expect("Couldn't write bindings!");
}
