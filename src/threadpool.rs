use std::{
    io::Error,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
};
pub struct ThreadPool {
    workers: Vec<Result<Worker, Error>>,
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
            Some(data) => match data.send(job) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error while Sending code: {}", e)
                }
            },
            None => {
                eprintln!("Could not send message")
            }
        }
        //unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!(
                "Shutting down worker {}",
                match worker {
                    Ok(id) => {
                        id.get_id()
                    }
                    Err(e) => {
                        return eprintln!("Unexpected error: {}", e);
                    }
                }
            );

            let _thread = match worker {
                Ok(worker) => worker.get_thread(),
                Err(_) => {
                    return eprintln!("Unexpected error occurred");
                }
            };
        }
    }
}

/// Declaring the worker struct
struct Worker {
    id: usize,
    thread: Rc<Option<thread::JoinHandle<()>>>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    /// The worker gets a job from the pool and executes and send the response back to the thread
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Error> {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
            .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job(); // This function executes the job and sends the response to the next thread
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Ok(Worker {
            id,
            thread: Some(thread).into(),
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    fn get_thread(&self) -> Rc<Option<thread::JoinHandle<()>>> {
        self.thread.clone()
    }
}
