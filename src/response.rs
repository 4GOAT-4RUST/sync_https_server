use std::io::Write;
/// Sends an HTTP response to the client.
pub fn send_response<T: Write>(stream: &mut T, response: &str) {
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Error writing response: {}", e);
    }

    if let Err(e) = stream.flush() {
        // Print an error message if flushing fails.
        eprintln!("Error flushing response: {}", e);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_send_response() {
        let mut output = Vec::new();
        send_response(&mut output, "HTTP/1.1 200 OK\r\n\r\n");
        assert_eq!(output, b"HTTP/1.1 200 OK\r\n\r\n");
    }
}
