use crate::{
    error::Error as StdError,
    ffi::{OsStr, OsString},
    fmt, io,
    marker::PhantomData,
    path::{self, PathBuf},
};

pub fn errno() -> i32 {
    panic!("should not be used on this target");
}

pub fn error_string(_errno: i32) -> String {
    panic!("should not be used on this target");
}

pub fn getcwd() -> io::Result<PathBuf> {
    let cwd = theseus_shim::getcwd();
    Ok(cwd.into())
}

pub fn chdir(path: &path::Path) -> io::Result<()> {
    theseus_shim::chdir(path.to_str().ok_or(theseus_shim::Error::InvalidFilename)?);
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
    // Theseus doesn't have the concept of a `PATH` environment variable.
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
    todo!();
}

pub struct Env {
    _inner: (),
}

impl Iterator for Env {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<(OsString, OsString)> {
        todo!();
    }
}

pub fn env() -> Env {
    todo!();
}

pub fn getenv(key: &OsStr) -> Option<OsString> {
    let key = rstr(key).ok()?;
    Some(theseus_shim::getenv(key)?.into())
}

pub fn setenv(key: &OsStr, value: &OsStr) -> io::Result<()> {
    let key = rstr(key)?;
    let value = rstr(value)?;
    theseus_shim::setenv(key, value).map_err(|e| e.into())
}

pub fn unsetenv(key: &OsStr) -> io::Result<()> {
    let key = rstr(key)?;
    theseus_shim::unsetenv(key).map_err(|e| e.into())
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
    // TODO: Lazy?
    s.to_str().ok_or(theseus_shim::Error::InvalidInput.into())
}
