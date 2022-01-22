use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    println!("got {}", rx.recv().unwrap())
}
