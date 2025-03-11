use std::sync::mpsc;
use sync_https_server::threadpool_impl::ThreadPool;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[test]
fn test_thread_pool_creation() {
    let size = 4;
    let pool = ThreadPool::new(size);
    assert_eq!(pool.workers.len(), size);
}

//this function test if my program is executing a single job
#[test]
fn test_execute_single_job() {
    let size = 2;
    let pool = ThreadPool::new(size);

    let (tx, rx) = mpsc::channel();

    pool.execute(move || {
        match tx.send(()) {
            Ok(_) => {println!("Send with success")},
            Err(e) => {eprintln!("Failed to send job: {}",e)},
        };
    });

    // Wait for the job to complete
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(_) => {println!("Execution successful")},
        Err(e) => {eprintln!("Error: {}",e)},
    };
}

#[test]
fn test_execute_multiple_jobs() {
    let size = 2;
    let pool = ThreadPool::new(size);

    let (tx, rx) = mpsc::channel();

    let  value = tx.send(());
    for _ in 0..size {
        pool.execute(move || {
            match value {
                Ok(_) => {println!("Thread executing ..")},
                Err(e) => {eprintln!("Failed to execute job: {}",e)},
            };
        });
    }

    // Wait for all jobs to complete
    for _ in 0..size {
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(_) => {println!("Jobs Completed")},
            Err(e) => {eprintln!("Jobs did not complete successfully: {}",e)},
        };
    }

    // Ensure no more jobs are in the queue
    assert_eq!(rx.try_recv().is_err(), true);
}

#[test]
fn test_execute_with_panic() {
    let size = 2;
    let pool = ThreadPool::new(size);

    let (tx, rx) = mpsc::channel();

    // Submit a job that panics
    pool.execute(move || {
        tx.send(()).unwrap();
        panic!("This job should panic");
    });

    // Submit a normal job


    // Wait for the normal job to complete
    match rx.recv_timeout(Duration::from_secs(2)) {
        Ok(_) => {println!("Finished waiting")},
        Err(e) => {eprintln!("Failed to wait: {}",e)},
    };

    // Ensure the panicking job did not block the normal job
    assert_eq!(rx.try_recv().is_err(), true);
}

#[test]
fn test_thread_pool_shutdown() {
    let size = 2;
    let pool = ThreadPool::new(size);

    let (tx, rx) = mpsc::channel();

    let  value = tx.send(());
    for _ in 0..size {
        pool.execute(move || {
            match value {
                Ok(_) => {println!("Data send !")},
                Err(e) => {eprintln!("Could not send data: {}",e)},
            };
        });
    }

    // Drop the pool to shut down the workers
    drop(pool);

    // Wait for all jobs to complete
    for _ in 0..size {
        match rx.recv_timeout(Duration::from_secs(3)) {
            Ok(_) => {println!("Job completed successfully")},
            Err(e) => {eprintln!("Could not complete Jobs: {}",e)},
        };
    }

    // Ensure no more jobs are in the queue
    assert_eq!(rx.try_recv().is_err(), true);
}