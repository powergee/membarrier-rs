extern crate bindgen;
extern crate cc;
extern crate libc;
extern crate cfg_if;

cfg_if::cfg_if! {
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

        /// Determine the right strategy to use on the current machine.
        fn main() {
            // Emit a right compile time flag for each cases.
            if membarrier::is_supported() {
                println!("cargo:rustc-cfg=membarrier_cfg");
            } else if mprotect::is_supported() {
                println!("cargo:rustc-cfg=mprotect_cfg");
            }
        }
    } else {
        /// For other environments, we don't have to do anything.
        fn main() {}
    }
}
