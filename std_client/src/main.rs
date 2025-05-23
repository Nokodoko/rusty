use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let message = "Are you there?";

    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();

    println!("Server response: {}", response);
}
