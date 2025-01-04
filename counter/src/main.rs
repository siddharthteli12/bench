use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::hint::black_box;

fn main() {
    let now = Instant::now();
    black_box(shared_counter_box());
    println!("TIME TAKEN BY BOX: {:?}", now.elapsed().as_micros());

    let now = Instant::now();
    black_box(shared_counter_arc());
    println!("TIME TAKEN BY ARC: {:?}", now.elapsed().as_micros());
}

// Using Box leak.
pub fn shared_counter_box() {
    let counter: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));
    let mut handlers = Vec::new();

    for _ in 0..1000 {
        let handler = thread::spawn(|| {
            for _ in 0..10000 {
                let mut lock = counter.lock().unwrap();
                *lock += 1;
            }
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

// Using Arc
pub fn shared_counter_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();

    for _ in 0..1000 {
        let counter = counter.clone();
        let handler = thread::spawn(move || {
            for _ in 0..10000 {
                let mut lock = counter.lock().unwrap();
                *lock += 1;
            }
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}
