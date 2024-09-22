use std::{
    iter::Map,
    net::{TcpListener, TcpStream},
};

struct Client {
    read_buffer: Vec<u8>,
    username: String,
    connection: bool, // idk the type here
}

struct Server {
    clients: Map<String, Client>,
    listener: TcpListener,
}

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
