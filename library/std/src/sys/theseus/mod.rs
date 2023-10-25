#![deny(unsafe_op_in_unsafe_fn)]

pub use theseus_shim as _;

impl From<theseus_shim::Error> for crate::io::Error {
    fn from(value: theseus_shim::Error) -> Self {
        use crate::io::ErrorKind;
        use theseus_shim::Error;

        match value {
            Error::NotFound => ErrorKind::NotFound,
            Error::PermissionDenied => ErrorKind::PermissionDenied,
            Error::ConnectionRefused => ErrorKind::ConnectionRefused,
            Error::ConnectionReset => ErrorKind::ConnectionReset,
            Error::HostUnreachable => ErrorKind::HostUnreachable,
            Error::NetworkUnreachable => ErrorKind::NetworkUnreachable,
            Error::ConnectionAborted => ErrorKind::ConnectionAborted,
            Error::NotConnected => ErrorKind::NotConnected,
            Error::AddrInUse => ErrorKind::AddrInUse,
            Error::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            Error::NetworkDown => ErrorKind::NetworkDown,
            Error::BrokenPipe => ErrorKind::BrokenPipe,
            Error::AlreadyExists => ErrorKind::AlreadyExists,
            Error::WouldBlock => ErrorKind::WouldBlock,
            Error::NotADirectory => ErrorKind::NotADirectory,
            Error::IsADirectory => ErrorKind::IsADirectory,
            Error::DirectoryNotEmpty => ErrorKind::DirectoryNotEmpty,
            Error::ReadOnlyFilesystem => ErrorKind::ReadOnlyFilesystem,
            Error::FilesystemLoop => ErrorKind::FilesystemLoop,
            Error::StaleNetworkFileHandle => ErrorKind::StaleNetworkFileHandle,
            Error::InvalidInput => ErrorKind::InvalidInput,
            Error::InvalidData => ErrorKind::InvalidData,
            Error::TimedOut => ErrorKind::TimedOut,
            Error::WriteZero => ErrorKind::WriteZero,
            Error::StorageFull => ErrorKind::StorageFull,
            Error::NotSeekable => ErrorKind::NotSeekable,
            Error::FilesystemQuotaExceeded => ErrorKind::FilesystemQuotaExceeded,
            Error::FileTooLarge => ErrorKind::FileTooLarge,
            Error::ResourceBusy => ErrorKind::ResourceBusy,
            Error::ExecutableFileBusy => ErrorKind::ExecutableFileBusy,
            Error::Deadlock => ErrorKind::Deadlock,
            Error::CrossesDevices => ErrorKind::CrossesDevices,
            Error::TooManyLinks => ErrorKind::TooManyLinks,
            Error::InvalidFilename => ErrorKind::InvalidFilename,
            Error::ArgumentListTooLong => ErrorKind::ArgumentListTooLong,
            Error::Interrupted => ErrorKind::Interrupted,
            Error::Unsupported => ErrorKind::Unsupported,
            Error::UnexpectedEof => ErrorKind::UnexpectedEof,
            Error::OutOfMemory => ErrorKind::OutOfMemory,
            Error::Other => ErrorKind::Other,
        }
        .into()
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
