use std::net::TcpListener;

use crate::{query::handle_client, threadpool::ThreadPool};

pub fn listener() {
    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(tcp_listener) => tcp_listener, // Successfully binds
        Err(e) => {
            eprintln!("Error binding to port: {}", e);
            return;
        }
    };
    println!("Server listening on 127.0.0.1:8080");

    let threadpool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed To Established Connection: {}", e);
                continue;
            }
        };

        threadpool.execute(move || {
            handle_client(stream);
        });
    }
}
