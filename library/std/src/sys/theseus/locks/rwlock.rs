use crate::sys::locks::Condvar;
use libtheseus::spin;

/// A writer-preferred reader-writer lock.
///
/// The implementation is based on the sgx and hermit implementations of
/// reader-writer locks.
#[derive(Debug, Default)]
pub struct RwLock {
    state: spin::Mutex<State>,
    readers: Condvar,
    writers: Condvar,
}

pub type MovableRwLock = RwLock;

impl RwLock {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Self {
        Self {
            state: spin::Mutex::new(State::Unlocked),
            readers: Condvar::new(),
            writers: Condvar::new(),
        }
    }

    #[inline]
    pub unsafe fn read(&self) {
        let mut state = self.state.lock();
        while !state.inc_readers() {
            // SAFETY: state corresponds to self.state.
            state = unsafe { self.readers.wait_spin(&self.state, state) };
        }
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        let mut state = self.state.lock();
        state.inc_readers()
    }

    #[inline]
    pub unsafe fn write(&self) {
        let mut state = self.state.lock();
        while !state.inc_writers() {
            // SAFETY: state corresponds to self.state.
            state = unsafe { self.readers.wait_spin(&self.state, state) };
        }
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        let mut state = self.state.lock();
        state.inc_writers()
    }

    /// Unlocks previously acquired shared access to this lock.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the current thread does not have shared access.
    #[inline]
    pub unsafe fn read_unlock(&self) {
        let mut state = self.state.lock();
        // if we were the last reader
        if state.dec_readers() {
            unsafe { self.writers.notify_one() };
        }
    }

    /// Unlocks previously acquired exclusive access to this lock.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the current thread does not have exclusive
    /// access.
    #[inline]
    pub unsafe fn write_unlock(&self) {
        let mut state = self.state.lock();
        state.dec_writers();
        // if no writers were waiting for the lock
        if unsafe { !self.writers.notify_one() } {
            unsafe { self.readers.notify_all() };
        }
    }
}

#[derive(Clone, Debug)]
enum State {
    Unlocked,
    Reading(usize),
    Writing,
}

impl Default for State {
    fn default() -> Self {
        Self::Unlocked
    }
}

impl State {
    fn inc_readers(&mut self) -> bool {
        match *self {
            State::Unlocked => {
                *self = State::Reading(1);
                true
            }
            State::Reading(ref mut count) => {
                *count += 1;
                true
            }
            State::Writing => false,
        }
    }

    fn inc_writers(&mut self) -> bool {
        match *self {
            State::Unlocked => {
                *self = State::Writing;
                true
            }
            State::Reading(_) | State::Writing => false,
        }
    }

    fn dec_readers(&mut self) -> bool {
        let zero = match *self {
            State::Reading(ref mut count) => {
                *count -= 1;
                *count == 0
            }
            State::Unlocked | State::Writing => {
                panic!("attempted to decrement readers in non-reader state")
            }
        };
        if zero {
            *self = State::Unlocked;
        }
        zero
    }

    fn dec_writers(&mut self) {
        match *self {
            State::Writing => {}
            State::Unlocked | State::Reading(_) => {
                panic!("attempted to decrement writers in non-writer state")
            }
        }
        *self = State::Unlocked;
    }
}
