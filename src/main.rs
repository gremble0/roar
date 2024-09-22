use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
    println!("Incoming connection")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
