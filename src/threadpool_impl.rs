use std::thread;
#[derive(Debug)]
/// This Thread pool will allow you to create any specified number if thread you wish to create
/// Each of the threads are running on an open closure
pub struct ThreadPool {
    workers: Vec<Workers>,
}
#[derive(Debug)]
pub struct Workers {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Workers {
    pub fn new(id: usize) -> Workers {
        let thread = thread::spawn(|| {});
        Workers { id, thread }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        if size < 1 {
            return ThreadPool::new(0);
        }
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Workers::new(id));
        }
        ThreadPool { workers }
    }
}
