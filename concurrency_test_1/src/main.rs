
use std::thread;
use std::time::Duration;

fn main() {

    let mut x = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("thread: hello number {}", i);
            println!("x = {:?}", x);
            thread::sleep(Duration::from_millis(10));
        }
    });

    // we cannot push because the thread moved its data
    // x.push(1);

    handle.join().unwrap();

    for i in 10..15 {
        println!("main: hello number {}", i);
        thread::sleep(Duration::from_millis(10));
    }
}
