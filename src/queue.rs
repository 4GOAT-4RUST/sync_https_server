use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    tasks: Arc<Mutex<VecDeque<Task>>>,
}
impl ThreadPool {
    pub fn new(size: usize, workers: Vec<thread::JoinHandle<()>> ) -> Self {
        //let size = 8;
        let tasks = Arc::new(Mutex::new(VecDeque::<Task>::new()));

        for _ in 0..size {
            let tasks = Arc::clone(&tasks);
            let worker: thread::JoinHandle<()> = thread::spawn(move || loop {
                let task = tasks.lock().unwrap().pop_front();
                if let Some(task) = task  {
                    task();
                }
             

            });

            workers.push(worker);

        }    
        ThreadPool { tasks, workers }
    }
}
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(Box::new(f));
    }
}
