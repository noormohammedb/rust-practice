use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: \n{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (http_status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let html_content = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        http_status_line,
        html_content.len(),
        html_content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/* HTTP RESPONSE

   HTTP-Version Status-Code Reason-Phrase CRLF
   headers CRLF
   message-body

   eg: HTTP/1.1 200 OK\r\n\r\n
*/
