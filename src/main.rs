mod base64;
mod handler;
mod threadpool;
use handler::handle_client;
use std::net::TcpListener;
use threadpool::ThreadPool;

fn main() {
    println!("############################################################");
    println!("##                                                        ##");
    println!("##            Welcome To 4GOATS SERVER                    ##");
    println!("##                                                        ##");
    println!("############################################################");

    let listener = match TcpListener::bind("0.0.0.0:8080") {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => {
            eprintln!("Error binding to port: {}", e);
            return;
        }
    };
    println!("Server listening on 127.0.0.1:8080");

    let threadpool = match ThreadPool::new(8) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("ThreadPool initialization failed: {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed To Establish Connection: {}", e);
                continue;
            }
        };

        threadpool.execute(move || {
            if let Err(e) = handle_client(&stream) {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}

mod base64;
mod handler;
mod threadpool;
