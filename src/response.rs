use std::io::Write;

pub fn send_response<T: Write>(stream: &mut T, response: &str) {
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error writing response: {}", e);
    }
    
    if let Err(e) = stream.flush() {
        eprintln!("Error flushing response: {}", e);
    }
}
