use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn send_response(stream: &mut std::net::TcpStream, response: &str) {
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}


pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    // Trying to read from Stream
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read form Stream: {}", e);
        return;
    }

    // Converting the buffer to String
    let request = match str::from_utf8(&buffer) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed To convert buffer to UTF-8: {}", e);
            let response = "HTTP/1.1 404 Bad Request\r\n\r\nError: Invalid UTF-8 Encoding.";
            send_response(&mut stream, response);
            return;
        }
    };

    println!("Received Request: \n{}", request);

    // Screening The Request Line
    let request_line = match request.lines().next() {
        Some(line) => line,
        // incase there's nothing in the line
        None => {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\n.";
            send_response(&mut stream, response);
            return;
        }
    };

    // Extract The query string From the Request line
    let query_path = match request_line.split_whitespace().nth(1) {
        Some(qp) => qp,
        None => "",
    };

    let query_string = match query_path.split('?').nth(1) {
        Some(qs) => qs,
        None => "",
    };

    // Parsing the Query Parameters

    let mut delay: Option<u64> = None;
    let payload: Option<String> = None;

    for pair in query_string.split('&') {
        let mut key_value = pair.split('=');
        let key = match key_value.next() {
            Some(k) => k,
            None => "",
        };

        let value = match key_value.next() {
            Some(val) => val,
            None => "",
        };

        match key {
            "delay" => {
                delay = value.parse::<u64>().ok();
            }

            "payload" => {
                value.to_string();
            }

            _ => {}
        }
    }

    // Validating Payload and Delay
    let delay = match delay {
        Some(d) if d > 0 => d,
        _ => {
            let response =
                "HTTP/1.1 400 Bad Request\r\n\r\nError: 'delay' must be a positive integer.";
            send_response(&mut stream, response);
            return;
        }
    };

    let payload = match payload {
        Some(p) if !p.trim().is_empty() => p,
        _ => {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\nError: 'payload' must be a non-empty string.";
            send_response(&mut stream, response);
            return;
        }
    };

    // Assuming Parameters are Valid
    println!("Received valid Request: delay={} , payload={}",delay,payload);
    let response = "HTTP/1.1 200 OK\r\n\r\nRequest processed successfully.";
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response; {}",e);
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to Flush response: {}",e)
    }
}
