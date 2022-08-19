use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::sys::unsupported;
use crate::time::Duration;

pub struct Thread(libtheseus::task::TaskRef);

pub const DEFAULT_MIN_STACK_SIZE: usize =
    libtheseus::mem::KERNEL_STACK_SIZE_IN_PAGES * libtheseus::mem::PAGE_SIZE;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let mmi_ref = libtheseus::mem::get_kernel_mmi_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "couldn't get kernel mmi"))?;
        let stack =
            libtheseus::task::alloc_stack_by_bytes(stack, &mut mmi_ref.lock().page_table)
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "couldn't allocate stack"))?;

        let child_task = libtheseus::task::new_task_builder(|_| p(), ())
            .block()
            .stack(stack)
            .spawn()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // FIXME: We need to delete the streams when the thread exits.
        let current_task_io_streams =
            libtheseus::stdio::get_streams(libtheseus::task::get_my_current_task_id().unwrap())
                .unwrap();
        libtheseus::stdio::insert_child_streams(child_task.id, current_task_io_streams);
        
        child_task.unblock();

        Ok(Thread(child_task))
    }

    pub fn yield_now() {
        libtheseus::task::yield_now();
    }

    pub fn set_name(name: &CStr) {
        let task = libtheseus::task::get_my_current_task().expect("couldn't get current task");
        let name = String::from_utf8_lossy(name.to_bytes()).to_string();
        task.set_name(name)
    }

    pub fn sleep(_dur: Duration) {
        panic!("can't sleep");
    }

    pub fn join(self) {
        self.0.join().expect("failed to join to task")
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
