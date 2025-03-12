use std::io::Write;

// Error Handling Respond
pub fn send_response(stream: &mut std::net::TcpStream, response: &str) {
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}
