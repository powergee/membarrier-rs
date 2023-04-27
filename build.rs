extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

/// Build heavy barrier C library for Apple environment.
fn main() {
    // Building custom barrier library must be conducted
    // only in MacOS-based systems.
    if !cfg!(any(target_os = "macos", target_os = "ios")) {
        return;
    }

    let libdir_path = PathBuf::from("apple")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let sources_path = libdir_path.join("barrier.c");
    let headers_path = libdir_path.join("barrier.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `barrier` library. Cargo will
    // automatically know it must look for a `libbarrier.a` file.
    println!("cargo:rustc-link-lib=barrier");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    cc::Build::new()
        .file(&sources_path)
        .opt_level(3)
        .compile("barrier");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .use_core()
        .allowlist_function("is_supported")
        .allowlist_function("flush_process_write_buffers")
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/barrier.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("barrier.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
