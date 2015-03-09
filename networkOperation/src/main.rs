// TCP Client using Rust
use std::old_io::TcpStream;

fn main() {
    println!("TCP Client Connecting to 127.0.0.1:8080...");
    // Creating a TCP socket connecting at 127.0.0.1:8080
    let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();

    // Sending a GET request with some URL param
    socket.write_all(b"GET /maidsafe.png HTTP/1.0\n\n");

    // Response from serve is captured
    let response = socket.read_to_end();

    // Printing the response
    println!("{:?}", response);
}
