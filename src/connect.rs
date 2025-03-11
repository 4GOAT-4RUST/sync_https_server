use crate::mili;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
pub fn listener() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        // let listener = TcpListener::bind("127.0.0.1:8080").unwrap
        Ok(tcp_listener) => tcp_listener,
        Err(e) => {
            eprintln!("failed to initialise listener: {}", e);
            std::process::exit(1);
        }
    };
    println!("Server listening on 127.0.0.1:7800");

    for stream in listener.incoming() {
        //  let stream = stream.map_err(|e|{
        //     eprintln!("no connection establish:{}", e);
        //     connectivity(stream);
        //     e
        //  })?;

        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed To Established Conection:{}", e);
                continue;
            }
        };
        connectivity(stream)
    }
}

pub fn connectivity(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    //  let http_request = reader.lines().next().map_err(|e|{
    //     eprintln!("the request is worong:{}", http_request)
    let http: Vec<_> = reader
        .lines()
        .map(|result| match result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("result is not valid:{}", e);
                "Error".to_string()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();
    if let Some(first_line) = http.first() {
        if first_line == "GET / HTTP/1.1" {
            match mili::mili() {
                Ok(s) => {
                    let status_line = "HTTP/1.1 200 OK";

                    if let Err(e) = stream.write_all(status_line.as_bytes()) {
                        eprintln!("Failed to write response: {}", e);
                    }
                }
                Err(e) => eprintln!("not possible"),
            }
        }
    }
}

// fn main(){f

// let denominator = 5;
//  if denominator == 0.map_err(|e|{{
//    eprintln!("this is wrong)")})}
//  }
