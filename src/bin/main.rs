use std::{fs, thread};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4); /// numbers of spawned threads.

    /// processing all incoming requests.
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     println!("Connection established");
    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }

    /// processing just first two incoming requests and then shutdown.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Connection established");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Response HTTP-Version ,Status code, reason-phrase, crlf, headers clrf, message-body
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1. 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1. 200 OK", "index.html")
        } else {
            ("HTTP/1.1. 404 NOT FOUND", "404.html")
        };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
