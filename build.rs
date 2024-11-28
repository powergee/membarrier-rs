#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    extern crate bindgen;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    println!("cargo::rustc-check-cfg=cfg(register_pointer_values)");

    let bindings = bindgen::Builder::default()
        .use_core()
        .header(
            "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/mach/thread_state.h",
        )
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("thread_state.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    let generated =
        fs::read_to_string(out_path).expect("Couldn't read the generated binding file!");

    if generated.contains("thread_get_register_pointer_values") {
        println!("cargo:rustc-cfg=register_pointer_values");
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {}
