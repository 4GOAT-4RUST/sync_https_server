use std::{
    sync::{mpsc, Arc, Mutex}, thread
};

use crate::worker::{Job, Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.as_ref() {
            Some(data) => {match data.send(job) {
                Ok(_) => {},
                Err(e) => {eprintln!("Error while Sending code: {}",e)},
            }},
            None => {eprintln!("Could not send message")},
        }
        //unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.get_id());

            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => {println!("All Good")},
                    Err(e) => {eprintln!("Not all Good: {:?}",e)},
                }
            }
        }
    }
}

