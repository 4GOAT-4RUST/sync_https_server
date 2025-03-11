use std::fmt;

pub fn log_success<T: fmt::Display>(message: T) {
    println!("[Success]: {}", message);
}

pub fn log_failure<F: fmt::Display>(message: F) {
    eprintln!("[Failure]: {}", message);
}
