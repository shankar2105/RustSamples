use std::old_io::TcpStream;

fn main() {
    println!("TCP Client Connecting to 127.0.0.1:8000...");
    let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
    socket.write_all(b"GET /maidsafe.png HTTP/1.0\n\n");
    let response = socket.read_to_end();
    println!("{:?}", response);
}
