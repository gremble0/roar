use std::{
    io::Result,
    iter::Map,
    net::{TcpListener, TcpStream},
};

struct Client {
    read_buffer: Vec<u8>,
    username: String,
    connection: TcpStream,
}

impl Client {
    fn new(stream: TcpStream) -> Client {}

    fn run() {
        loop {}
    }
}

struct Server {
    clients: Map<String, Client>,
    listener: TcpListener,
}

impl Server {
    fn new() -> Server {}

    fn listen() {
        loop {}
    }

    fn accept(new_stream: TcpStream) {}
}

fn handle_client(stream: TcpStream) -> Client {
    println!("Incoming connection");

    Client {
        read_buffer: Vec::new(),
        username: String::new(),
        connection: stream,
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1337")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
