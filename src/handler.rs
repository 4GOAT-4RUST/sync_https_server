use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::Duration;
use std::{str, thread};

use crate::base64::base64_decode;
use std::fmt;

#[derive(Debug)]
pub enum RequestError {
    InvalidRequestLine,
    InvalidDelay,
    MissingPayload,
    InvalidBase64,
    IoError(std::io::Error),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::InvalidRequestLine => write!(f, "Error: Invalid Request Line"),
            RequestError::InvalidDelay => write!(f, "Error: 'delay' must be a positive integer"),
            RequestError::MissingPayload => {
                write!(f, "Error: 'payload' must be a non-empty string")
            }
            RequestError::InvalidBase64 => write!(f, "Error: Invalid base64 payload"),
            RequestError::IoError(err) => write!(f, "I/O Error: {}", err),
        }
    }
}

impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> Self {
        RequestError::IoError(err)
    }
}

/// Handles incoming client requests and processes them accordingly.
pub fn handle_client<T: Read + Write>(mut stream: T) -> Result<(), RequestError> {
    let mut buffer = [0; 1024];

    // Read the request from the stream
    let bytes_read = stream.read(&mut buffer).map_err(RequestError::IoError)?;
    let request =
        str::from_utf8(&buffer[..bytes_read]).map_err(|_| RequestError::InvalidRequestLine)?;

    let mut lines = request.lines();
    let request_line = lines.next().ok_or(RequestError::InvalidRequestLine)?;

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 {
        eprintln!("Invalid request format: {:?}", parts);
        return Err(RequestError::InvalidRequestLine);
    }

    let method = parts[0];
    let path = parts[1];
    let http_version = parts[2];

    if http_version != "HTTP/1.1" {
        eprintln!("Unsupported HTTP version: {}", http_version);
        return Err(RequestError::InvalidRequestLine);
    }

    // Check if GET request has the required query parameters
    if method == "GET" {
        // Ensure the query has a payload
        if !path.contains("payload=") {
            eprintln!("Missing payload in query: {}", path);
            return Err(RequestError::MissingPayload);
        }

        // Call the handler for GET requests
        handle_decode(&mut stream, path)?;
    } else {
        eprintln!("Unsupported HTTP Method: {}", method);
        return Err(RequestError::InvalidRequestLine);
    }

    Ok(())
}

/// Handles decoding requests, extracts parameters, and sends a response.
fn handle_decode<T: Write>(stream: &mut T, request: &str) -> Result<(), RequestError> {
    let params = parse_query_params(request);

    // Extract delay or return an error
    let delay = params
        .get("delay")
        .and_then(|v| v.parse::<u64>().ok())
        .ok_or(RequestError::InvalidDelay)?;

    // Extract payload or return an error
    let payload = params.get("payload").ok_or(RequestError::MissingPayload)?;

    // Try to decode the payload
    match base64_decode(payload) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);

            // Log decoded payload
            println!("{}# - {}ms: {}", payload, delay, decoded_str);

            // Delay the response
            thread::sleep(Duration::from_millis(delay));

            // Send ONLY the Base64 payload back to the client
            send_response(stream, payload);

            Ok(())
        }
        Err(_) => Err(RequestError::InvalidBase64),
    }
}

// Parses query parameters from the request path
fn parse_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(start) = query.find('?') {
        let query_str = &query[start + 1..];
        for pair in query_str.split('&') {
            let mut split = pair.split('=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                params.insert(key.to_string(), value.to_string());
            }
        }
    }

    params
}

fn send_response<T: Write>(stream: &mut T, body: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        Content-Type: text/plain\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        body.len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Test a valid request with correct query parameters.
    #[test]
    fn test_handle_client_valid_request() {
        let request = "GET /?payload=SGVsbG8=&delay=10 HTTP/1.1\r\n\r\n"; // "SGVsbG8=" is "Hello"
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(result.is_ok());
    }

    /// Test an invalid request without a valid request line.
    #[test]
    fn test_handle_client_invalid_request_line() {
        let request = "INVALID REQUEST";
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(matches!(result, Err(RequestError::InvalidRequestLine)));
    }

    /// Test a request missing the HTTP version.
    #[test]
    fn test_handle_client_missing_query_params() {
        let request = "GET / HTTP/1.1\r\n\r\n"; // No payload or delay query params
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);

        assert!(
            matches!(result, Err(RequestError::MissingPayload)),
            "Expected MissingPayload error, but got {:?}",
            result
        );
    }

    /// Test a request with an invalid Base64 payload.
    #[test]
    fn test_handle_client_invalid_base64() {
        let request = "GET /?payload=%%%INVALID_BASE64%%%&delay=10 HTTP/1.1\r\n\r\n"; // Invalid Base64
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(
            matches!(result, Err(RequestError::InvalidBase64)),
            "Expected InvalidBase64 error, got {:?}",
            result
        );
    }

    /// Test a request with missing query parameters.
    #[test]
    fn test_handle_client_missing_query_params2() {
        let request = "GET / HTTP/1.1\r\n\r\n";
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(matches!(result, Err(RequestError::MissingPayload)));
    }

    /// Test a request with a valid payload but missing delay.
    #[test]
    fn test_handle_client_missing_delay() {
        let request = "GET /?payload=SGVsbG8= HTTP/1.1\r\n\r\n"; // Missing delay
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(
            matches!(result, Err(RequestError::InvalidDelay)),
            "Expected InvalidDelay error, got {:?}",
            result
        );
    }

    /// Test a request where delay is not a valid number.
    #[test]
    fn test_handle_client_invalid_delay() {
        let request = "GET /?payload=SGVsbG8=&delay=xyz HTTP/1.1\r\n\r\n"; // Invalid delay
        let mut stream = Cursor::new(request.as_bytes().to_vec());
        let result = handle_client(&mut stream);
        assert!(
            matches!(result, Err(RequestError::InvalidDelay)),
            "Expected InvalidDelay error, got {:?}",
            result
        );
    }

    #[test]
    fn test_request_error_display() {
        let error = RequestError::InvalidRequestLine;
        assert_eq!(format!("{}", error), "Error: Invalid Request Line");
    }

    #[test]
    fn test_request_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "io failure");
        let request_error: RequestError = io_error.into();
        assert!(matches!(request_error, RequestError::IoError(_)));
    }
}
