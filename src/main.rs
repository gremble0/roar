use std::{
    collections::HashMap,
    fmt, io,
    net::{SocketAddr, TcpListener, TcpStream},
};

struct Client {
    read_buffer: Vec<u8>,
    username: String,
    connection: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        todo!()
    }

    pub fn run() {
        todo!()
    }
}

struct Server {
    clients: HashMap<SocketAddr, Client>,
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u16) -> Result<Server, io::Error> {
        Ok(Server {
            clients: HashMap::new(),
            listener: TcpListener::bind(format!("127.0.0.1:{}", port))?,
        })
    }

    pub fn listen(&mut self) {
        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => self.accept(stream, addr),
                Err(err) => eprintln!("ERROR: could not accept connection: {}", err),
            }
        }
    }

    fn accept(&mut self, stream: TcpStream, addr: SocketAddr) {
        self.clients.insert(addr, Client::new(stream));
    }
}

fn main() {
    let mut server = Server::new(1337).unwrap();
    server.listen();
}
