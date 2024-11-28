extern crate bindgen;
extern crate cfg_if;
extern crate libc;
use cfg_if::cfg_if;

fn main() {
    cfg_if! {
        if #[cfg(target_os = "linux")] {
            mod membarrier {
                /// Call the `sys_membarrier` system call.
                #[inline]
                fn sys_membarrier(cmd: libc::c_int) -> libc::c_long {
                    unsafe { libc::syscall(libc::SYS_membarrier, cmd, 0 as libc::c_int) }
                }

                /// Returns `true` if the `sys_membarrier` call is available.
                pub fn is_supported() -> bool {
                    // Queries which membarrier commands are supported. Checks if private expedited
                    // membarrier is supported.
                    let ret = sys_membarrier(libc::MEMBARRIER_CMD_QUERY);
                    if ret < 0
                        || ret & libc::MEMBARRIER_CMD_PRIVATE_EXPEDITED as libc::c_long == 0
                        || ret & libc::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED as libc::c_long
                            == 0
                    {
                        return false;
                    }

                    // Tries registering the current process as a user of private expedited membarrier.
                    if sys_membarrier(libc::MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED) < 0 {
                        return false;
                    }

                    true
                }
            }

            mod mprotect {
                /// Returns `true` if the `mprotect`-based trick is supported.
                pub fn is_supported() -> bool {
                    cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64")
                }
            }

            println!("cargo::rustc-check-cfg=cfg(has_membarrier)");
            println!("cargo::rustc-check-cfg=cfg(has_mprotect)");

            // Emit a right compile time flag for each cases.
            if membarrier::is_supported() {
                println!("cargo:rustc-cfg=has_membarrier");
            } else if mprotect::is_supported() {
                println!("cargo:rustc-cfg=has_mprotect");
            }
        } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
            // Parse `mach/thread_state.h` and generate FFI bindings.
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
    }
}
