use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection Established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // this buffer is 1024 bytes long
    stream.read(&mut buffer).unwrap();
    // we are populating buffer with data from the stream
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // printing contents of buffer
    // from_utf8_lossy => Converts a slice of bytes to a string, including invalid characters.

    let contents = fs::read_to_string("index.html").unwrap();
    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
