// compile-flags: -Zmiri-thread-liveness=1

use std::thread::spawn;

fn fair() {
    let j1 = spawn(|| {
        for i in 0..10 {
            println!("Thread 1 : {}",i);
        }
    });
    let j2 = spawn(|| {
        for i in 0..10 {
            println!("Thread 2 : {}",i);
        }
    });
    j1.join().unwrap();
    j2.join().unwrap();
}

fn main() {
    fair();
    println!("Reset");
    fair();
    println!("Reset");
    fair();
}