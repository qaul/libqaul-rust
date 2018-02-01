extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {

    let mut dst = Config::new("qaul.net")
                 .define("GUI", "CLI")
                 .build();

    dst.push("build");
    dst.push("src");
    dst.push("libqaul");

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=libqaul");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=libqaul");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // .header("qaul.net/")
        // Finish the builder and generate the bindings.
        // .generate()
        // Unwrap the Result and panic on failure.
        // .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}