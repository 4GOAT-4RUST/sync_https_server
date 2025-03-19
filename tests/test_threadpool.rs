#[cfg(test)]
mod tests {
    use sync_https_server::threadpool::ThreadPool;

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
    #[should_panic] // This tells Rust that we expect a panic
    fn test_thread_pool_creation_failure() {
        let _ = ThreadPool::new(0); // Should panic due to assert!(size > 0)
    }
}
