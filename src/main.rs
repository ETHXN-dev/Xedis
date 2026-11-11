#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];

                // continuously read commands in the same connection
                loop {
                    let _n = match stream.read(&mut buffer) {
                        Ok(0) => break, // client closed the connection
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("read error: {}", e);
                            break;
                        }
                    };

                    let response = b"+PONG\r\n";
                    if let Err(e) = stream.write_all(response) {
                        eprintln!("write error: {}", e);
                        break;
                    };
                }
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
