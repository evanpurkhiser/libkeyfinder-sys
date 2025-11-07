fn main() {
    // Locate the system-installed libkeyfinder using pkg-config
    let lib = pkg_config::Config::new()
        .atleast_version("2.2")
        .probe("libkeyfinder")
        .expect("Could not find libkeyfinder");

    // Build the C++ bridge with cxx
    let mut build = cxx_build::bridge("src/lib.rs");

    build
        .file("src/bridge.cpp")
        .flag("-std=c++11");

    // Add include paths from libkeyfinder
    for path in &lib.include_paths {
        build.include(path);
    }

    build.compile("libkeyfinder-sys");

    // Tell cargo to rerun if bridge files change
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
}
