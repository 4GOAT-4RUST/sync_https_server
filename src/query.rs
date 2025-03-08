use core::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    base64_decode::base64_decode,
    response::{self, send_response},
};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Trying to read from Stream
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                eprintln!("No data recieved from Client");
            }

            // Converting the buffer to String
            let request = match str::from_utf8(&buffer[..bytes_read]) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Failed To convert buffer to UTF-8: {}", e);
                    let response = "HTTP/1.1 404 Bad Request\r\n\r\nError: Invalid UTF-8 Encoding.";
                    send_response(&mut stream, response);
                    return;
                }
            };

            println!("Received Request: \n{}", request);
            // check if Request is a Post for decoding
            if request.starts_with("POST /decode HTTP/1.1") {
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
                        let response =
                            "HTTP/1.1 400 Bad Request\r\n\r\nError: 'payload' must be a non-empty string.";
                        send_response(&mut stream, response);
                        return;
                    }
                };

                let decoded_payload = match base64_decode(&payload) {
                    Ok(data) => data,
                    Err(e) => {
                        let response =  "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}";
                        send_response(&mut stream, response);
                        return;
                    }
                };

                // Convert the decoded payload back to a string (for simplicity)
                let decoded_message = String::from_utf8_lossy(&decoded_payload);

                // Construct the response
                let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{{\"decoded_message\": \"{}\"}}",
            decoded_message.len() + 22, /* 22 is the length of the JSON structure and quotes */
            decoded_message
        );

                send_response(&mut stream, &response);
                // Assuming Parameters are Valid
                println!(
                    "Received valid Request: delay={} , payload={}",
                    delay, payload
                );
                let response = "HTTP/1.1 200 OK\r\n\r\nRequest processed successfully.";
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response; {}", e);
                }
                if let Err(e) = stream.flush() {
                    eprintln!("Failed to Flush response: {}", e)
                }
            } else {
                // Construct a 404 respnse
                let response =  "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: 9\r\n\r\nNot Found";
                send_response(&mut stream, response);
            }
        }

        Err(e) => {
            eprintln!("Failed to read form Stream: {}", e);
        }
    }
}
