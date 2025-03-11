use std::collections::HashMap;
use std::io::{Read, Write};

use crate::base64_decode::base64_decode;
use crate::response::send_response;

// Common HTTP response status codes used in responses
const HTTP_200: &str = "HTTP/1.1 200 OK\r\n";
const HTTP_400: &str = "HTTP/1.1 400 Bad Request\r\n";
const HTTP_404: &str = "HTTP/1.1 404 Not Found\r\n";

/// Handles incoming client requests and processes them accordingly.
pub fn handle_client<T: Read + Write>(mut stream: T) {
    let mut buffer = [0; 1024]; // Buffer to store incoming request data (max 1024 bytes)

    // Read request data from the stream
    match stream.read(&mut buffer) {
        Ok(bytes_read) if bytes_read == 0 => { // If no data is received
            println!("Debug: No data received from client");
            send_response(&mut stream, &format!("{}No data received from Client\r\n", HTTP_400));
            return;
        }
        Ok(_) => {} // Request was read successfully, proceed further
        Err(_) => { // Handle read error
            println!("Debug: Error reading request");
            send_response(
                &mut stream,
                &format!("{}Error reading request\r\n", HTTP_400),
            );
            return;
        }
    }

    // Convert raw request bytes into a string
    let request = match std::str::from_utf8(&buffer) {
        Ok(r) => r,
        Err(_) => { // Handle UTF-8 conversion failure
            println!("Debug: Failed to convert buffer to UTF-8");
            send_response(
                &mut stream,
                &format!("{}Failed to convert buffer to UTF-8\r\n", HTTP_400),
            );
            return;
        }
    };

    println!("Debug: Received request: {}", request);
    let mut lines = request.lines();

    // Extract the request line (first line of the HTTP request)
    let request_line = match lines.next() {
        Some(line) => line,
        None => { // If request line is missing, return an error
            println!("Debug: Empty request line");
            send_response(
                &mut stream,
                &format!("{}Error: Invalid Request Line\r\n", HTTP_400),
            );
            return;
        }
    };

    // Split request line into parts: Method, URL, and HTTP version
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 || parts[2] != "HTTP/1.1" { // Ensure request format is correct
        println!("Debug: Invalid request line: {}", request_line);
        send_response(
            &mut stream,
            &format!("{}Error: Invalid Request Line\r\n", HTTP_400),
        );
        return;
    }

    // Determine request method and route, then handle accordingly
    match (parts.get(0), parts.get(1)) {
        (Some(&"POST"), Some(&"/decode")) => handle_decode(&mut stream, request), // Handle decode requests
        (_, Some(route)) => { // Handle unknown routes
            println!("Debug: Route not found: {}", route);
            send_response(
                &mut stream,
                &format!("{}Content-Length: 9\r\n\r\nNot Found", HTTP_404),
            );
        }
        _ => { // Handle malformed requests
            println!("Debug: Malformed request");
            send_response(
                &mut stream,
                &format!("{}Error: Malformed request\r\n", HTTP_400),
            );
        }
    }
}

/// Handles decoding requests by extracting parameters and decoding the payload.
fn handle_decode<T: Write>(stream: &mut T, request: &str) {
    let params = parse_query_params(request); // Extract key-value parameters from request body
    println!("Debug: Parsed params: {:?}", params);

    // Extract 'delay' parameter and ensure it is a positive integer
    let _delay = match params.get("delay").and_then(|v| v.parse::<u64>().ok()) {
        Some(d) if d > 0 => d,
        _ => {
            send_response(
                stream,
                &format!("{}Error: 'delay' must be a positive integer\r\n", HTTP_400),
            );
            return;
        }
    };
    
    // Extract 'payload' parameter and ensure it is not empty
    let payload = match params.get("payload").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => {
            println!("Debug: Missing 'payload'");
            send_response(
                stream,
                &format!("{}Error: 'payload' must be a non-empty string\r\n", HTTP_400),
            );
            return;
        }
    };

    // Attempt to decode the base64-encoded payload
    match base64_decode(payload) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded); // Convert decoded bytes to string
            let response = format!(
                "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                HTTP_200, decoded_str.len(), decoded_str
            );
            send_response(stream, &response);
        }
        Err(_) => { // Handle invalid base64 data
            println!("Debug: Invalid base64 payload");
            send_response(
                stream,
                &format!("{}Error: Invalid base64 payload\r\n", HTTP_404),
            );
        }
    }
}

/// Extracts and organizes query parameters from the request body.
fn parse_query_params(request: &str) -> HashMap<&str, &str> {
    let mut params = HashMap::new();
    if let Some(body) = request.split("\r\n\r\n").nth(1) { // Extract the request body
        for pair in body.split('&') { // Split body into key-value pairs
            let mut kv = pair.splitn(2, '=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                params.insert(k, v.trim_matches(char::from(0))); // Remove any null characters
            }
        }
    }
    params
}
