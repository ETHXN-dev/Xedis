#![allow(unused_imports)]
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        };

        let resp_command = resp_parser(&buffer[..bytes_read]);

        let response = if resp_command[0].eq_ignore_ascii_case("echo") && resp_command.len() > 1 {
            &encode_as_bulk_str(resp_command[1])
        } else {
            "+PONG\r\n"
        };

        if let Err(e) = stream.write_all(response.as_bytes()).await {
            eprintln!("Write error: {}", e);
            break;
        }
    }
}

fn resp_parser(resp_array: &[u8]) -> Vec<&str> {
    // *2\r\n$794\r\nECHO\r\n$3\r\nhey\r\n => ["ECHO", "hey"]

    let array_as_str: &str = std::str::from_utf8(resp_array).unwrap();

    let n_elements: usize = array_as_str[1..array_as_str.find('\r').unwrap()]
        .parse()
        .unwrap();

    // track the start position of the length of each bulk string
    let mut len_start = array_as_str.find('$').unwrap() + 1;
    (0..n_elements)
        .map(|_| {
            let rest_of_str = &array_as_str[len_start..];
            let len_end = rest_of_str.find('\r').unwrap();
            let string_len: usize = rest_of_str[..len_end].parse().unwrap();

            len_start += len_end + 2 + string_len + 3;
            &rest_of_str[len_end + 2..len_end + 2 + string_len]
        })
        .collect()
}

fn encode_as_bulk_str(s: &str) -> String {
    format!("${}\r\n{}\r\n", s.len(), s)
}
