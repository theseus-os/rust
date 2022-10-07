use super::{current_task, current_task_id, io_err, unsupported};
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
    let cwd = current_task()?.get_env().lock().cwd().into();
    Ok(cwd)
}

pub fn chdir(path: &path::Path) -> io::Result<()> {
    current_task()?
        .get_env()
        .lock()
        .chdir(&libtheseus::path::Path::new(
            path.to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "path not UTF-8 valid"))?
                .to_owned(),
        ))
        .map_err(|e| match e {
            libtheseus::env::Error::NotADirectory => io::Error::new(
                io::ErrorKind::NotADirectory,
                "tried to change directory into node that isn't a directory",
            ),
            libtheseus::env::Error::NotFound => io::Error::new(
                io::ErrorKind::NotFound,
                "tried to change directory into node that doesn't exist",
            ),
        })
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
    let task = current_task()?;
    let app_crate = task
        .app_crate
        .as_ref()
        .ok_or_else(|| io_err("task didn't contain reference to app crate"))?;
    let path = app_crate
        .lock_as_ref()
        .object_file
        .lock()
        .get_absolute_path();
    Ok(path.into())
}

pub struct Env {
    inner: libtheseus::env::EnvIter,
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        self.inner.next().map(|(k, v)| (k.into(), v.into()))
    }
}

pub fn env() -> Env {
    let task = current_task().expect("couldn't get current task");
    Env {
        inner: task.get_env().lock().variables.clone().into_iter(),
    }
}

pub fn getenv(key: &OsStr) -> Option<OsString> {
    let task = libtheseus::task::get_my_current_task().expect("couldn't get current task");
    task.get_env()
        .lock()
        .get(key.to_str().expect("key was not valid unicode"))
        .map(|s| s.into())
}

pub fn setenv(key: &OsStr, value: &OsStr) -> io::Result<()> {
    let task = current_task()?;
    task.get_env().lock().set(
        key.to_str()
            .ok_or_else(|| invalid_data_io_err("key was not valid unicode"))
            .to_owned(),
        value
            .to_str()
            .ok_or_else(|| invalid_data_io_err("value was not valid unicode"))
            .to_owned(),
    );
    Ok(())
}

pub fn unsetenv(key: &OsStr) -> io::Result<()> {
    let task = current_task()?;
    task.get_env().lock().unset(
        key.to_str()
            .ok_or_else(|| invalid_data_io_err("key was not valid unicode")),
    );
    Ok(())
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    let task = current_task().expect("couldn't get current task");
    task.mark_as_exited(Box::new(code))
        .expect("couldn't mark task as exited");
    libtheseus::task::yield_now();

    panic!("task scheduled after exiting");
}

pub fn getpid() -> u32 {
    current_task_id().expect("couldn't get current task id") as u32
}

fn invalid_data_io_err(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
