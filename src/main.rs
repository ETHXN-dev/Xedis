#![allow(unused_imports)]
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = b"+PONG\r\n";
                stream.write_all(response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
