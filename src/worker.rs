use std::{sync::{mpsc, Arc, Mutex}, thread::{self}};

pub struct Worker {
    id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    /// The worker gets a job from the pool and executes and send the response back to the thread
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(val) => {val},
                Err(e) => {eprintln!("Error: {}",e);
            return;},
            }
            .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();// This function executes the job and sends the response to the next thread
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




