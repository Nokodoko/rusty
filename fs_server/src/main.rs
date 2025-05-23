use std::io::Result;
use std::net::{SocketAddr, TcpListener, TcpStream};

// TODO:
// 1. PRINT A SERVER HEADER/FLAG WHEN STARTING UP THE SERVER
// 2. HANDLE ROUTES (DOUBLE CHECK SOME OF THE API CALLS YOU ARE ALREADY USING IN RUST)
// 3. ADD TOKIO ASYNC (FOR MULTIPLE ASSIST WITH MULTIPLE CONNECITONS -- SEE IF YOU CAN DO NATIVELY WITH STD LIB)
// 4. INCLUDE DATABASE CONNECTION AND LOOK FOR AVAILABLE DATABASE DRIVERS (POSTGRES)
// 5. MIGRATE DD_LIB (test aws api callbacks -> fargate/ec2/{eks?})
// 6. TEST ANISBLE CALLBACKS

fn handler_func(stream: TcpStream) {
    println!("FS SERVER")
}

fn main() -> Result<()> {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    let addrs = [
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 443),
    ];

    let listener = TcpListener::bind(&addrs[..])?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handler_func(stream);
            }
            Err(e) => println!("Failed Connection {}", e),
        }
    }
    Ok(())
}
