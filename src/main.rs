use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..10 {
        let c1 = Arc::clone(&c);
        let handle = thread::spawn(move || {
            let mut v = c1.lock().unwrap();

            *v += 1;
        });

        hs.push(handle);
    }

    for h in hs {
        h.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap());
}
