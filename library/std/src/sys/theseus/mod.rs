#![deny(unsafe_op_in_unsafe_fn)]

pub use libtheseus as _;

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

fn io_err(s: &str) -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other, s)
}

fn current_task_id() -> crate::io::Result<usize> {
    libtheseus::task::get_my_current_task_id().ok_or_else(|| io_err("couldn't get current task id"))
}

fn current_task() -> crate::io::Result<&'static libtheseus::task::TaskRef> {
    libtheseus::task::get_my_current_task().ok_or_else(|| io_err("couldn't get current task"))
}

impl From<libtheseus::core2::io::Error> for crate::io::Error {
    fn from(e: libtheseus::core2::io::Error) -> crate::io::Error {
        use libtheseus::core2;

        let kind = match e.kind() {
            core2::io::ErrorKind::NotFound => crate::io::ErrorKind::NotFound,
            core2::io::ErrorKind::PermissionDenied => crate::io::ErrorKind::PermissionDenied,
            core2::io::ErrorKind::ConnectionRefused => crate::io::ErrorKind::ConnectionRefused,
            core2::io::ErrorKind::ConnectionReset => crate::io::ErrorKind::ConnectionReset,
            core2::io::ErrorKind::ConnectionAborted => crate::io::ErrorKind::ConnectionAborted,
            core2::io::ErrorKind::NotConnected => crate::io::ErrorKind::NotConnected,
            core2::io::ErrorKind::AddrInUse => crate::io::ErrorKind::AddrInUse,
            core2::io::ErrorKind::AddrNotAvailable => crate::io::ErrorKind::AddrNotAvailable,
            core2::io::ErrorKind::BrokenPipe => crate::io::ErrorKind::BrokenPipe,
            core2::io::ErrorKind::AlreadyExists => crate::io::ErrorKind::AlreadyExists,
            core2::io::ErrorKind::WouldBlock => crate::io::ErrorKind::WouldBlock,
            core2::io::ErrorKind::InvalidInput => crate::io::ErrorKind::InvalidInput,
            core2::io::ErrorKind::InvalidData => crate::io::ErrorKind::InvalidData,
            core2::io::ErrorKind::TimedOut => crate::io::ErrorKind::TimedOut,
            core2::io::ErrorKind::WriteZero => crate::io::ErrorKind::WriteZero,
            core2::io::ErrorKind::Interrupted => crate::io::ErrorKind::Interrupted,
            core2::io::ErrorKind::Other => crate::io::ErrorKind::Other,
            core2::io::ErrorKind::UnexpectedEof => crate::io::ErrorKind::UnexpectedEof,
            _ => crate::io::ErrorKind::Uncategorized,
        };

        match e.into_inner() {
            Some(s) => crate::io::Error::new(kind, s),
            None => crate::io::Error::from(kind),
        }
    }
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
