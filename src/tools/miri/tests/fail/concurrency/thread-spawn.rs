//@only-windows: Only Windows is not supported.

use std::thread;

//@error-pattern: can't create threads on Windows

fn main() {
    thread::spawn(|| {});
}
