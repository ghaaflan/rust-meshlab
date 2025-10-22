use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the build directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir).join("c_wrapper/build");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=meshlab_api");

    // Tell cargo to invalidate the built crate whenever wrapper changes
    println!("cargo:rerun-if-changed=c_wrapper/include/meshlab_api.h");
    println!("cargo:rerun-if-changed=c_wrapper/src/meshlab_api.cpp");
}
