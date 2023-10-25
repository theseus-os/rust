use super::io_err;
use crate::io;
use libtheseus::{
    core2::io::{Read, Write},
    stdio::{stderr, stdin, stdout},
};

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let stdin = stdin().map_err(io_err)?;
        let mut lock = stdin.lock();
        lock.read(buf).map_err(io::Error::from)
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let stdout = stdout().map_err(io_err)?;
        let mut lock = stdout.lock();
        lock.write(buf).map_err(io::Error::from)
    }

    fn flush(&mut self) -> io::Result<()> {
        let stdout = stdout().map_err(io_err)?;
        let mut lock = stdout.lock();
        lock.flush().map_err(io::Error::from)
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let stderr = stderr().map_err(io_err)?;
        let mut lock = stderr.lock();
        lock.write(buf).map_err(io::Error::from)
    }

    fn flush(&mut self) -> io::Result<()> {
        let stderr = stderr().map_err(io_err)?;
        let mut lock = stderr.lock();
        lock.flush().map_err(io::Error::from)
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    None
}
