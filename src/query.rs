use std::collections::HashMap;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::base64_decode::base64_decode;
use crate::response::send_response;

// Common HTTP response status codes
const HTTP_200: &str = "HTTP/1.1 200 OK\r\n";
const HTTP_400: &str = "HTTP/1.1 400 Bad Request\r\n";
const HTTP_404: &str = "HTTP/1.1 404 Not Found\r\n";

/// Handles incoming client requests and processes them accordingly.
pub fn handle_client<T: Read + Write>(mut stream: T) {
    let mut buffer = [0; 1024]; // A small storage space to read the request data

    // Try reading data from the stream
    match stream.read(&mut buffer) {
        Ok(0) => {
            println!("Debug: No data received from client");
            send_response(&mut stream, "No data received from Client\n");
            return;
        }
        Ok(_) => {} // Data was received, continue processing
        Err(_) => {
            println!("Debug: Error reading request");
            send_response(
                &mut stream,
                &format!("{}Error reading request\r\n", HTTP_400),
            );
            return;
        }
    }

    // Convert the raw request data into a readable string
    let request = match std::str::from_utf8(&buffer) {
        Ok(r) => r,
        Err(_) => {
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

    // Get the first line of the request (e.g., "POST /decode HTTP/1.1")
    let request_line = match lines.next() {
        Some(line) => line,
        None => {
            println!("Debug: Empty request line");
            send_response(
                &mut stream,
                &format!("{}Error: Invalid Request Line\r\n", HTTP_400),
            );
            return;
        }
    };

    // Split the request line into its parts
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 || parts[2] != "HTTP/1.1" {
        println!("Debug: Invalid request line: {}", request_line);
        send_response(
            &mut stream,
            &format!("{}Error: Invalid Request Line\r\n", HTTP_400),
        );
        return;
    }

    // Figure out what kind of request we received
    match (parts.first(), parts.get(1)) {
        (Some(&"POST"), Some(&"/decode")) => handle_decode(&mut stream, request),
        (_, Some(route)) => {
            println!("Debug: Route not found: {}", route);
            send_response(
                &mut stream,
                &format!("{}Content-Length: 9\r\n\r\nNot Found", HTTP_404),
            );
        }
        _ => {
            println!("Debug: Malformed request");
            send_response(
                &mut stream,
                &format!("{}Error: Malformed request\r\n", HTTP_400),
            );
        }
    }
}

/// Handles decoding requests, extracts parameters, and sends a response.
fn handle_decode<T: Write>(stream: &mut T, request: &str) {
    let params = parse_query_params(request);
    println!("Debug: Parsed params: {:?}", params);

    // Extract 'delay' from request parameters and ensure it's a valid number
    let delay = match params.get("delay").and_then(|v| v.parse::<u64>().ok()) {
        Some(d) if d > 0 => d,
        _ => {
            println!("Debug: Invalid or missing 'delay'");
            send_response(
                stream,
                &format!("{}Error: 'delay' must be a positive integer\r\n", HTTP_400),
            );
            return;
        }
    };

    // Extract 'payload' from request parameters and make sure it's not empty
    let payload = match params.get("payload").filter(|p| !p.is_empty()) {
        Some(p) => p,
        None => {
            println!("Debug: Missing 'payload'");
            send_response(
                stream,
                &format!(
                    "{}Error: 'payload' must be a non-empty string\r\n",
                    HTTP_400
                ),
            );
            return;
        }
    };

    // Attempt to decode the Base64 payload
    match base64_decode(payload) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);
            let response_body = format!("Decoded Message: {}\nDelay: {}", decoded_str, delay);
            let content_length = response_body.len();

            let response = format!(
                "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                HTTP_200, content_length, response_body
            );

            println!("Debug: Successful decoding, response: {}", response);

            // Introduce the delay before sending the response
            println!("Debug: Delaying response by {} seconds...", delay);
            thread::sleep(Duration::from_secs(delay));

            // Send the response after the delay
            send_response(stream, &response);
        }
        Err(_) => {
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
