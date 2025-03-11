use std::net::TcpListener;

use crate::{query::handle_client, threadpool_impl::ThreadPool};
pub fn listener() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(tcp_listener) => tcp_listener, // Successfully binds
        Err(e) => {
            eprintln!("Error binding to port: {}", e);
            return;
        }
    };
    println!("Server listening on 127.0.0.1:7800");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed To Established Conection:{}", e);
                continue;
            }
        };

        let threadpool = ThreadPool::new(8);

        threadpool.execute(move || loop {
            handle_client(match stream.try_clone() {
                Ok(r) => r,
                Err(e) => {
                    return eprintln!("Error: {}", e);
                }
            });
        });
    }
}
