fn main() {
    // Skip actual build on docs.rs - just generate the bridge code
    if std::env::var("DOCS_RS").is_ok() {
        // Generate the cxx bridge for documentation
        let _ = cxx_build::bridge("src/lib.rs");
        println!("cargo:warning=Building for docs.rs - skipping libkeyfinder linkage");
        return;
    }

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
