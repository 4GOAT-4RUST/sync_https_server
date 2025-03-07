use std::{sync::{mpsc, Arc, Mutex}, thread::{self, Thread}};

pub struct Worker {
    id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

