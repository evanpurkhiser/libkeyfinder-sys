use std::env;
use std::path::PathBuf;

fn main() {
    // Locate the system-installed libkeyfinder using pkg-config
    let lib = pkg_config::Config::new()
        .atleast_version("2.2")
        .probe("libkeyfinder")
        .expect("Could not find libkeyfinder");

    // Tell Rust to link against libkeyfinder
    for path in &lib.include_paths {
        println!("cargo:include={}", path.display());
    }

    // Generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-std=c++17")
        .clang_args(
            lib.include_paths
                .iter()
                .map(|p| format!("-I{}", p.display())),
        )
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the output directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
