use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size < 1 {
            return Err("The size of the thread pool cannot be less than one");
        }
        let (sender, receiver) = mpsc::channel();
<<<<<<< HEAD
<<<<<<< HEAD
        // This creates a channel between the sender and the receiver
=======
        // This creates a channel between the sender and the receiver 
>>>>>>> 71b0f77 (accepted changes from remote branch)
=======
        // This creates a channel between the sender and the receiver
>>>>>>> 76f3eac (test: added unit tests)

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(match Worker::new(id, Arc::clone(&receiver)) {
                Ok(worker) => worker,
                Err(_) => return Err("Error could get worker"),
            });
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
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
            println!("Shutting down worker {}", worker.get_id() + 1);

            if let Some(thread) = worker.thread.take() {
<<<<<<< HEAD
<<<<<<< HEAD
                match thread.join() {
                    // we join the other threads that have not been drop so that they finish their execution and are also drop before the worker can be drop
=======
                match thread.join() { // we join the other threads that have not been drop so that they finish their execution and are also drop before the worker can be drop 
>>>>>>> 71b0f77 (accepted changes from remote branch)
=======
                match thread.join() {
                    // we join the other threads that have not been drop so that they finish their execution and are also drop before the worker can be drop
>>>>>>> 76f3eac (test: added unit tests)
                    Ok(_) => {
                        println!("Successfully Executed The Job")
                    }
                    Err(e) => {
                        eprintln!("Could Not Complete Job: {:?}", e)
                    }
                }
            }
        }
    }
}

/// Implementing worker
pub struct Worker {
    id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    /// The worker gets a job from the pool and executes and send the response back to the thread
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, &'static str> {
        // here we loop so as to allow other incoming request to be spawn on the same thread
        let thread = thread::spawn(move || loop {
            // let _message = {
<<<<<<< HEAD
<<<<<<< HEAD
=======
                let reciever = match receiver.lock() {
                    Ok(val) => val.recv(),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return ;
                    }
                };
>>>>>>> 71b0f77 (accepted changes from remote branch)

=======
>>>>>>> 76f3eac (test: added unit tests)
            let reciever = match receiver.lock() {
                Ok(val) => val.recv(),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 76f3eac (test: added unit tests)
            };

            match reciever {
                Ok(job) => {
<<<<<<< HEAD
                    println!("Worker {} got a job; executing.", id + 1);
=======
                    println!("Worker {id} got a job; executing.");
>>>>>>> 76f3eac (test: added unit tests)

                    job(); // This function executes the job and sends the response to the next thread
                }
                Err(_) => {
<<<<<<< HEAD
                    println!("Worker {} disconnected; shutting down.", id + 1);
                    break;
                }
            }

=======
>>>>>>> 71b0f77 (accepted changes from remote branch)
=======
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
>>>>>>> 76f3eac (test: added unit tests)
            // };
        });

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let size = 4;
        let pool = ThreadPool::new(size).expect("Failed to create thread pool");
        assert_eq!(pool.workers.len(), size);
    }

    #[test]
    fn test_execute_single_job() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let (tx, rx) = mpsc::channel();

        pool.execute(move || {
            tx.send(()).expect("Failed to send signal");
        });

        assert!(rx.recv_timeout(Duration::from_secs(2)).is_ok());
    }

    #[test]
    fn test_execute_multiple_jobs() {
        let pool = ThreadPool::new(3).expect("Failed to create thread pool");

        let (tx, rx) = mpsc::channel();
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..5 {
            let tx_clone = tx.clone();
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().expect("Failed to lock counter");
                *num += 1;
                tx_clone.send(()).expect("Failed to send signal");
            });
        }

        for _ in 0..5 {
            assert!(rx.recv_timeout(Duration::from_secs(2)).is_ok());
        }

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 5);
    }

    #[test]
    fn test_execute_with_panic() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let (tx, rx) = mpsc::channel();

        pool.execute(move || {
            let _ = tx.send(()); // Attempt to send signal before panic
            panic!("This job should panic");
        });

        assert!(
            rx.recv_timeout(Duration::from_secs(2)).is_ok(),
            "Expected at least one successful execution before panic"
        );
    }

    #[test]
    fn test_thread_pool_shutdown() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let (tx, rx) = mpsc::channel();

        for _ in 0..2 {
            let tx_clone = tx.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_millis(100)); // Simulate work
                tx_clone.send(()).expect("Failed to send signal");
            });
        }

        drop(pool); // Drop the pool, should shutdown workers

        assert!(rx.recv_timeout(Duration::from_secs(2)).is_ok());
        assert!(rx.recv_timeout(Duration::from_secs(2)).is_ok());

        // Ensure all jobs completed and no extra messages are in the queue
        assert!(rx.try_recv().is_err());
    }
    #[test]
    fn test_thread_pool_creation_failure() {
        let _ = ThreadPool::new(0); // Should panic due to assert!(size > 0)
    }
}
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> 242b154 (style{clean up} removed all warning in threadpool.rs)

>>>>>>> 71b0f77 (accepted changes from remote branch)
=======
<<<<<<< Updated upstream
=======
>>>>>>> 242b154 (style{clean up} removed all warning in threadpool.rs)

=======
>>>>>>> Stashed changes
>>>>>>> 06944a3 (test: added unit tests)
=======
>>>>>>> 76f3eac (test: added unit tests)
