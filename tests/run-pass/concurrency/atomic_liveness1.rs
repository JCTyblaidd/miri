// compile-flags: -Zmiri-thread-liveness=1

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Barrier;
use std::sync::Arc;
use std::thread::spawn;

fn op(counter: &Arc<(Barrier,AtomicUsize)>) -> impl FnOnce() -> [usize; 10] {
    let counter = counter.clone();
    move || {
        let mut array = [0; 10];
        //counter.0.wait();
        for i in 0..10 {
            array[i] = counter.1.fetch_add(1, Ordering::SeqCst);
        }
        array
    }
}

fn fair() {
    let shared_counter = Arc::new((Barrier::new(5), AtomicUsize::new(1)));
    let j1 = spawn(op(&shared_counter));
    let j2 = spawn(op(&shared_counter));
    let j3 = spawn(op(&shared_counter));
    let j4 = spawn(op(&shared_counter));
    //shared_counter.0.wait();
    println!("Thread 1 => {:?}", j1.join().unwrap());
    println!("Thread 2 => {:?}", j2.join().unwrap());
    println!("Thread 3 => {:?}", j3.join().unwrap());
    println!("Thread 4 => {:?}", j4.join().unwrap());
}

fn main() {
    fair();
    println!("Reset");
    fair();
    println!("Reset");
    fair();
}