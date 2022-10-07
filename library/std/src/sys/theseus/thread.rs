use super::{current_task, current_task_id, io_err};
use crate::{ffi::CStr, io, num::NonZeroUsize, sys::unsupported, time::Duration};
use libtheseus::{mem, stdio, task};

pub struct Thread(task::JoinableTaskRef);

pub const DEFAULT_MIN_STACK_SIZE: usize = mem::KERNEL_STACK_SIZE_IN_PAGES * mem::PAGE_SIZE;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let mmi_ref = mem::get_kernel_mmi_ref().ok_or_else(|| io_err("couldn't get kernel mmi"))?;
        let stack = task::alloc_stack_by_bytes(stack, &mut mmi_ref.lock().page_table)
            .ok_or_else(|| io_err("couldn't allocate stack"))?;

        let child_task =
            task::new_task_builder(|_| p(), ()).block().stack(stack).spawn().map_err(io_err)?;

        // FIXME: We need to delete the streams when the thread exits.
        let current_task_io_streams = stdio::get_streams(current_task_id()?)
            .ok_or_else(|| io_err("couldn't get current task io streams"))?;
        stdio::insert_child_streams(child_task.id, current_task_io_streams);

        let current_env = current_task()?.get_env();
        child_task.set_env(current_env);

        child_task.unblock();
        Ok(Thread(child_task))
    }

    pub fn yield_now() {
        task::yield_now();
    }

    pub fn set_name(name: &CStr) {
        let task = current_task().expect("couldn't get current task");
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
