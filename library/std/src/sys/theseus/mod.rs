#![deny(unsafe_op_in_unsafe_fn)]

pub use libtheseus as _;

// TODO: Don't generate _start / don't call this function for Theseus.
#[no_mangle]
#[doc(hidden)]
pub unsafe extern "C" fn __libc_start_main(
    main: extern "C" fn(i32, *const *const u8, *const *const u8) -> i32,
    argc: i32,
    argv: *const *const u8,
    _init_dummy: extern "C" fn(),
    _fini_dummy: extern "C" fn(),
    _ldso_dummy: extern "C" fn(),
) -> i32 {
    // FIXME: Does Rust use envp?
    main(argc, argv, crate::ptr::null()) 
}

#[no_mangle]
pub unsafe extern "C" fn runtime_entry() -> i32 {
    0
}

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod env;
pub mod fs;
pub mod io;
pub mod locks;
pub mod net;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod time;

mod common;
pub use common::*;
