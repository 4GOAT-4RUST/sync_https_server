use std::collections::HashMap;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::base64_decode::base64_decode;
use crate::error::RequestError;
use crate::response::send_response;

// Common HTTP response status codes
const HTTP_200: &str = "HTTP/1.1 200 OK\r\n";
const HTTP_404: &str = "HTTP/1.1 404 Not Found\r\n";

/// Handles incoming client requests and processes them accordingly.
pub fn handle_client<T: Read + Write>(mut stream: T) -> Result<(), RequestError> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).map_err(RequestError::IoError)?;

    let request = std::str::from_utf8(&buffer).map_err(|_| RequestError::InvalidRequestLine)?;

    println!("Received request: {}", request);

    let mut lines = request.lines();
    let request_line = lines.next().ok_or(RequestError::InvalidRequestLine)?;

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 || parts[2] != "HTTP/1.1" {
        return Err(RequestError::InvalidRequestLine);
    }

    match (parts.first(), parts.get(1)) {
        (Some(&"GET"), Some(&"/decode")) => handle_decode(&mut stream, request)?,
        (_, Some(route)) => {
            send_response(
                &mut stream,
                &format!("{}Content-Length: 9\r\n\r\nNot Found", HTTP_404),
            );
        }
        _ => return Err(RequestError::InvalidRequestLine),
    }

    Ok(())
}

/// Handles decoding requests, extracts parameters, and sends a response.
fn handle_decode<T: Write>(stream: &mut T, request: &str) -> Result<(), RequestError> {
    let params = parse_query_params(request);
    println!("Parsed params: {:?}", params);

    let delay = params
        .get("delay")
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or(RequestError::InvalidDelay)?;

    let payload = params.get("payload").ok_or(RequestError::MissingPayload)?;

    match base64_decode(payload) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);
            let response_body = format!("Decoded Message: {}", decoded_str);
            let content_length = response_body.len();

            let response = format!(
                "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                HTTP_200, content_length, response_body
            );

            println!("Delaying response by {} milliseconds...", delay);
            thread::sleep(Duration::from_millis(delay));

            send_response(stream, &response);
            Ok(())
        }
        Err(_) => Err(RequestError::InvalidBase64),
    }
}

/// Extracts and organizes query parameters from the request body.
fn parse_query_params(request: &str) -> HashMap<&str, &str> {
    let mut params = HashMap::new();
    if let Some(body) = request.split("\r\n\r\n").nth(1) {
        for pair in body.split('&') {
            let mut kv = pair.splitn(2, '=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                params.insert(k, v.trim_matches(char::from(0))); // Remove null characters if any
            }
        }
    }
    params
}

mod tests {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_handle_client_valid_request() {
        let request = "GET / HTTP/1.1\r\n\r\n";
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_client_invalid_request_line() {
        let request = "INVALID REQUEST";
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(matches!(result, Err(RequestError::InvalidRequestLine)));
    }

    #[test]
    fn test_handle_client_missing_http_version() {
        let request = "GET / missing_version";
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(matches!(result, Err(RequestError::InvalidRequestLine)));
    }

    #[test]
    fn test_handle_client_base64_encoded_path() {
        let encoded_path = "L3Rlc3Q="; // Base64 for "/test"
        let request = format!("GET {} HTTP/1.1\r\n\r\n", encoded_path);
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(result.is_ok());
    }
}
