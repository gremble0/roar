use std::{
    collections::HashMap,
    fmt, io,
    net::{TcpListener, TcpStream},
};

struct Client {
    read_buffer: Vec<u8>,
    username: String,
    connection: TcpStream,
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        todo!()
    }

    fn run() {
        loop {}
    }
}

struct Server {
    clients: HashMap<String, Client>,
    listener: TcpListener,
}

#[derive(Debug)]
pub enum ServerError {
    BindError(io::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::BindError(err) => write!(f, "Failed to bind to address: {}", err),
        }
    }
}

impl Server {
    pub fn new(port: u16) -> Result<Server, ServerError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port));

        Ok(Server {
            clients: HashMap::new(),
            listener: listener.unwrap(), // TODO: LOL bad
        })
    }

    pub fn listen(&self) {
        loop {}
    }

    fn accept(&self, new_stream: TcpStream) {
        todo!()
    }
}

fn main() {
    let server = Server::new(1337).unwrap();
    server.listen();
}
