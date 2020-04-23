use std::thread;
use std::sync::mpsc;

#[allow(dead_code)]
pub fn run3() {
    let mut c = vec![];

    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread number: {}", i);
        }));
    }

    for j in c {
        #[allow(unused_must_use)]
        j.join();
    }
}

#[allow(dead_code)]
pub fn run2() {
    let c = vec![1, 2, 3];

    let _handle = thread::spawn(move || {
        println!("vector: {:?}", c);
    });

    // handle.join();
}

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { tx.send(42).unwrap(); });

    println!("got {}", rx.recv().unwrap());
}