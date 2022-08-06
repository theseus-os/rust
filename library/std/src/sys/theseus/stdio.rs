use crate::io;

use libtheseus::{
    core2::io::{Write, Read},
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
        let stdin= stdin().unwrap();
        let mut lock = stdin.lock();
        lock.read(buf).map_err(|_| io::Error::from(io::ErrorKind::Other))
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let stdout = stdout().unwrap();
        let mut lock = stdout.lock();
        lock.write(buf).map_err(|_| io::Error::from(io::ErrorKind::Other))
    }

    fn flush(&mut self) -> io::Result<()> {
        let stdout = stdout().unwrap();
        let mut lock = stdout.lock();
        lock.flush().map_err(|_| io::Error::from(io::ErrorKind::Other))
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let stderr = stderr().unwrap();
        let mut lock = stderr.lock();
        lock.write(buf).map_err(|_| io::Error::from(io::ErrorKind::Other))
    }

    fn flush(&mut self) -> io::Result<()> {
        let stderr = stderr().unwrap();
        let mut lock = stderr.lock();
        lock.flush().map_err(|_| io::Error::from(io::ErrorKind::Other))
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<Vec<u8>> {
    None
}
