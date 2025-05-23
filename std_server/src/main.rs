use std::{net::{TcpStream, TcpListener}, io::{Read, Write}, thread};
use std::io::Result;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0, 255];
    stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\nHello World!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Could not accept connection: {}", e);
            }
        }
    }
    
    Ok(())
}
