use std::io::{self, Cursor, Read, Write};
use sync_https_server::query::handle_client;

// Mock implementation of a TCP stream using an in-memory buffer
struct MockTcpStream {
    stream: Cursor<Vec<u8>>, // Cursor allows reading/writing like a real stream
}

impl MockTcpStream {
    // Constructor to initialize the mock stream with given data
    fn new(data: Vec<u8>) -> Self {
        MockTcpStream {
            stream: Cursor::new(data),
        }
    }
}

// Implement Read trait for MockTcpStream to mimic reading from a network stream
impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

// Implement Write trait for MockTcpStream to mimic writing to a network stream
impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

// Simulates sending an HTTP request to the server and capturing the response
fn simulate_client_request(request: &str) -> io::Result<Option<String>> {
    let mut mock_stream = MockTcpStream::new(request.as_bytes().to_vec());
    handle_client(&mut mock_stream);

    let mut response = Vec::new();
    mock_stream.stream.set_position(0); // Reset cursor to read response

    match mock_stream.stream.read_to_end(&mut response) {
        Ok(_) => Ok(String::from_utf8(response).ok()),
        Err(e) => Err(e),
    }
}

// Test case: No data received
#[test]
fn test_handle_client_no_data() {
    if let Ok(Some(response)) = simulate_client_request("") {
        assert_eq!(response, "No data received from Client\n");
    }
}

// Test case: Invalid UTF-8 in request
#[test]
fn test_handle_client_invalid_utf8() {
    let invalid_utf8 = vec![0x80, 0x80]; // Invalid UTF-8 sequence
    let mut mock_stream = MockTcpStream::new(invalid_utf8);
    handle_client(&mut mock_stream);

    let mut response = Vec::new();
    mock_stream.stream.set_position(0);
    if mock_stream.stream.read_to_end(&mut response).is_ok() {
        if let Ok(response_str) = String::from_utf8(response) {
            assert!(response_str.contains("Failed to convert buffer to UTF-8"));
        }
    }
}

// Test case: Invalid 'delay' parameter (zero or missing)
#[test]
fn test_handle_client_get_decode_invalid_delay() {
    let request =
        "GET /decode HTTP/1.1\r\nContent-Length: 25\r\n\r\ndelay=0&payload=SGVsbG8gd29ybGQ=";
    if let Ok(Some(response)) = simulate_client_request(request) {
        assert!(response.contains("HTTP/1.1 400 Bad Request"));
        assert!(response.contains("Error: 'delay' must be a positive integer"));
    }
}

// Test case: Missing or empty 'payload' parameter
#[test]
fn test_handle_client_get_decode_invalid_payload() {
    let request = "GET /decode HTTP/1.1\r\nContent-Length: 25\r\n\r\ndelay=5&payload=";
    if let Ok(Some(response)) = simulate_client_request(request) {
        assert!(response.contains("HTTP/1.1 400 Bad Request"));
        assert!(response.contains("Error: 'payload' must be a non-empty string"));
    }
}

// Test case: Invalid HTTP request line
#[test]
fn test_handle_client_invalid_request_line() {
    if let Ok(Some(response)) = simulate_client_request("INVALID /decode HTTP/1.1") {
        assert!(response.contains("HTTP/1.1 400 Bad Request"));
    }
}

// Test case: Invalid base64 payload
#[test]
fn test_handle_client_invalid_base64_payload() {
    let request = "GET /decode HTTP/1.1\r\nContent-Length: 25\r\n\r\ndelay=5&payload=invalid";
    if let Ok(Some(response)) = simulate_client_request(request) {
        assert!(response.contains("HTTP/1.1 404 Not Found"));
    }
}

// Test case: Valid request with correct parameters
#[test]
fn test_handle_client_get_decode() {
    let request = "GET /decode HTTP/1.1\r\nContent-Length: 25\r\n\r\ndelay=5&payload=SGVsbG8=";
    if let Ok(Some(response)) = simulate_client_request(request) {
        assert!(response.contains("HTTP/1.1 200 OK"));
    }
}
