
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // we have a transmitter and a receiver
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let x = thread::spawn(move || {
        // let val = String::from("hi from transmitter");
        for i in 1..10 {
            thread::sleep(Duration::from_millis(100));
            tx.send(i).unwrap();
        }
    });

    // we can capture the cloned tx1 here and send more data
    thread::spawn(move || {
        for i in 20..30 {
            thread::sleep(Duration::from_millis(100));
            tx1.send(i).unwrap();
        }
    });

    // recv will wait for the thread to return data
    /*
    loop {
        let received = rx.recv();
        match received {
            Ok(val) => println!("got data! {}", val),
            Err(_) => break
        }
    }
    */

    // this is a much simpler implementation of the above code
    for received in rx {
        println!("got data! {}", received);
    }

    println!("done with receiving!");

    // how to use mutex
    let m = Arc::new(Mutex::new(0));
    let n = Arc::new(Mutex::new)

    let mut handles = vec![];

    for i in 1..5 {
        let counter = Arc::clone(&m);
        let handle = thread::spawn(move || {
            println!("Thread {}: hi", i);
            let mut var = counter.lock().unwrap();
            *var += 5;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", m.lock().unwrap());
}
