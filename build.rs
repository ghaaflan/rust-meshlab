use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch == "wasm32" {
        // WASM build: link to Emscripten-compiled static library
        let lib_path = PathBuf::from(&manifest_dir).join("c_wrapper/build-wasm");

        println!("cargo:rustc-link-search=native={}", lib_path.display());
        println!("cargo:rustc-link-lib=static=meshlab_api");

        // Note: Don't link libc/libc++ for WASM - Emscripten handles this
    } else {
        // Native build: link to dynamic library
        let lib_path = PathBuf::from(&manifest_dir).join("c_wrapper/build");

        println!("cargo:rustc-link-search=native={}", lib_path.display());
        println!("cargo:rustc-link-lib=dylib=meshlab_api");
    }

    // Tell cargo to invalidate the built crate whenever wrapper changes
    println!("cargo:rerun-if-changed=c_wrapper/include/meshlab_api.h");
    println!("cargo:rerun-if-changed=c_wrapper/src/meshlab_api.cpp");
}
