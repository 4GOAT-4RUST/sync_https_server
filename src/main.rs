use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7800") {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => {
            eprintln!("failed to initialise listener");
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

fn connectivity(mut stream: TcpStream) {
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
            let line = match fs::read_to_string("hello.html") {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("no html file found");
                    return;
                }
            };
            let status_line = "HTTP/1.1 200 OK";
            let lenght = line.len();

            let answer = format!("{status_line}\r\nline lenght: {lenght}\r\n\r\n{line}");
            if let Err(e) = stream.write_all(answer.as_bytes()) {
                eprintln!("Failed to write response: {}", e);
            }
        }
    }
}
