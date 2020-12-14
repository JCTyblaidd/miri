// ignore-windows: No libc on Windows

use std::sync::{Arc, Mutex};
use std::thread::spawn;

extern "Rust" {
    fn miri_yield_thread();
}


fn main() {
    let shared = Arc::new(Mutex::new(0usize));
    let s1 = shared.clone();
    let mut s_guard = shared.lock().unwrap();
    let j1 = spawn(move || {
        let mut a_guard = loop {
            // yield loop for try-lock.
            if let Ok(guard) = s1.try_lock() {
                break guard
            }else{
                unsafe { miri_yield_thread(); } //~ERROR livelock
            }
        };
        *a_guard = 2;
    });

    j1.join().unwrap();
    *s_guard = 1;
}