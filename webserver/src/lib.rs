use std::thread::Thread;


pub struct ThreadPool {
    threads: Vec<Thread>
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> ThreadPool {
        let threads = vec![];
        threads.push(std::thread::spawn(|| {

        }));
        threads[0]
        ThreadPool {
        
        }
    }
}
