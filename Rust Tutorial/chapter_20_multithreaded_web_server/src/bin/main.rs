use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread::{self},
    time::Duration,
};

use chapter_20_multithreaded_web_server::ThreadPool;

fn main() {
    let mut counter = 1;
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    // this will able to run 4 threads

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {});
        handle_connection(stream, counter);
        counter += 1;
    }
}

fn handle_connection(mut stream: TcpStream, _counter: i32) {
    let mut buffer = [0; 1024]; // this buffer is 1024 bytes long
    stream.read(&mut buffer).unwrap();
    // we are populating buffer with data from the stream
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // printing contents of buffer
    // from_utf8_lossy => Converts a slice of bytes to a string, including invalid characters.

    let get = b"GET / HTTP/1.1\r\n";
    // by prefixing with string will give bytes array of our string
    // => [71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10]
    let sleep = b"GET /sleep HTTP/1.1\r\n"; // for multithreaded

    let (status_line, filaname) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        // this will make a thread sleep for 5 sec
        // when passed /sleep
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filaname).unwrap();
    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
/*
as_bytes() => Returns a byte slice of this String's contents.
The inverse of this method is [from_utf8].
*/

// if buffer.starts_with(get) {
//     // if our buffer starts with same string then we will return html page
//     let contents = fs::read_to_string("index.html").unwrap();
//     // let response = "HTTP/1.1 200 OK\r\n\r\n";

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{} {}",
//         contents.len(),
//         contents,
//         counter,
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
//     println!("{counter}");
// } else {
//     let status_line = "HTTP/1.1 404 NOT FOUND";
//     let contents = fs::read_to_string("404.html").unwrap();
//     let response = format!(
//         "{}\r\nContent-Length:{}\r\n\r\n{}",
//         status_line,
//         contents.len(),
//         contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
