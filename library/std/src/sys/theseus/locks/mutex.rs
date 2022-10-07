use libtheseus::{spin, task};

/// A mutex.
///
/// The implementation is based on [a Princeton University lecture][lecture].
///
/// [lecture]: https://www.cs.princeton.edu/courses/archive/fall16/cos318/lectures/6.MutexImplementation.pdf
#[derive(Debug, Default)]
pub struct Mutex {
    /// The inner state of a mutex.
    ///
    /// Using an IRQ safe mutex ensures even low priority tasks are able to
    /// complete their critical section. If preemption was enabled and a low
    /// priority task was preempted while holding onto the state, deadlock will
    /// occur if there are enough high priority tasks to not reschedule the
    /// low priority task, and one of the high priority task also tries to
    /// acquire the state.
    state: spin::Mutex<State>,
}

pub type MovableMutex = Mutex;

impl Mutex {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Mutex {
        Mutex {
            state: spin::Mutex::new(State::NEW),
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) {}

    #[inline]
    pub unsafe fn lock(&self) {
        let guard = task::hold_preemption();
        let mut state = self.state.lock();

        if !state.is_locked {
            state.is_locked = true;
            return;
        }

        let current_task = task::get_my_current_task()
            .expect("raw_mutex::Mutex::lock(): couldn't get current task");
        state.queue.push(current_task);
        current_task.block();

        drop(state);

        // Hypothetically a different core can unlock the mutex here, making the
        // yield_now unnecessary, but that doesn't impact the correctness of the code.

        drop(guard);

        // TODO: Yield to task holding lock?
        task::yield_now();

        // NOTE: We only reach here after the thread has been unblocked by
        // another thread.
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        let guard = task::hold_preemption();
        let mut state = self.state.lock();

        if state.queue.is_empty() {
            state.is_locked = false;
        } else {
            let task = state.queue.remove(0);
            task.unblock();
        }

        // Explicitly drop the inner mutex before enabling preemption.
        // NOTE: Rust implicitly drops them in the same order.
        drop(state);
        drop(guard);
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        let guard = task::hold_preemption();
        let mut state = self.state.lock();

        if state.is_locked {
            drop(state);
            drop(guard);
            false
        } else {
            state.is_locked = true;
            drop(state);
            drop(guard);
            true
        }
    }
}

#[derive(Clone, Debug, Default)]
struct State {
    is_locked: bool,
    // TODO: Ideally we'd use a VecDeque but that doesn't have a const initialiser. However, it's
    // not a particularly big deal since waitqueues are usually small.
    queue: Vec<&'static task::TaskRef>,
}

impl State {
    /// HACK: This exists because stable `const fn` can only call stable `const
    /// fn`, so they cannot call `Self::new()`.
    const NEW: Self = Self::new();

    pub const fn new() -> Self {
        Self {
            is_locked: false,
            queue: Vec::new(),
        }
    }
}
