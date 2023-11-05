use super::unsupported;
use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::fmt;
use crate::io;
use crate::marker::PhantomData;
use crate::path::{self, PathBuf};
use crate::sys::theseus::convert_err;

pub fn errno() -> i32 {
    panic!("not supported on this platform")
}

pub fn error_string(_errno: i32) -> String {
    panic!("not supported on this platform")
}

pub fn getcwd() -> io::Result<PathBuf> {
    Ok(theseus_shim::getcwd().into())
}

pub fn chdir(path: &path::Path) -> io::Result<()> {
    let path = path.to_str().ok_or_else(|| convert_err(theseus_shim::Error::InvalidFilename))?;
    theseus_shim::chdir(path).map_err(convert_err)?;
    Ok(())
}

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "not supported on this platform yet"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env(!);

impl Env {
    // FIXME(https://github.com/rust-lang/rust/issues/114583): Remove this when <OsStr as Debug>::fmt matches <str as Debug>::fmt.
    pub fn str_debug(&self) -> impl fmt::Debug + '_ {
        let Self(inner) = self;
        match *inner {}
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        match *inner {}
    }
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        let Self(inner) = self;
        match *inner {}
    }
}

pub fn env() -> Env {
    panic!("not supported on this platform")
}

pub fn getenv(key: &OsStr) -> Option<OsString> {
    let key = rstr(key).ok()?;
    Some(theseus_shim::getenv(key)?.into())
}

pub fn setenv(key: &OsStr, value: &OsStr) -> io::Result<()> {
    let key = rstr(key)?;
    let value = rstr(value)?;
    theseus_shim::setenv(key, value).map_err(convert_err)
}

pub fn unsetenv(key: &OsStr) -> io::Result<()> {
    let key = rstr(key)?;
    theseus_shim::unsetenv(key).map_err(convert_err)
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    theseus_shim::exit(code);
}

pub fn getpid() -> u32 {
    theseus_shim::getpid()
}

fn rstr(s: &OsStr) -> Result<&str, io::Error> {
    s.to_str().ok_or_else(|| convert_err(theseus_shim::Error::InvalidInput))
}
