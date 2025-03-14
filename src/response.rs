use std::io::Write;

/// Sends an HTTP response to the client over the provided stream
///
/// This function takes a mutable reference to any type that implements the `Write` trait
/// (such as a TCP stream) and writes the given response string to it
/// It ensures that the response is properly flushed, so the client receives it immediately
pub fn send_response<T: Write>(stream: &mut T, response: &str) {
    // Convert the response string into bytes and attempt to send it over the stream
    // `.write_all()` ensures that the entire response is written, not just part of it
    if let Err(e) = stream.write_all(response.as_bytes()) {
        // Print an error message if writing fails
        eprintln!("Error writing response: {}", e);
    }

    // Flush the stream to ensure all written data is sent immediately.
    // This prevents potential buffering delays that might cause incomplete responses.
    if let Err(e) = stream.flush() {
        // Print an error message if flushing fails.
        eprintln!("Error flushing response: {}", e);
    }
}
