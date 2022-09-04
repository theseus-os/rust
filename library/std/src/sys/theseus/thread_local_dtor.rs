#![unstable(feature = "thread_local_internals", issue = "none")]

pub unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    libtheseus::tls::register_dtor(t, dtor);
}
