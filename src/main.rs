use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
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
    clients: Arc<Mutex<HashMap<SocketAddr, Client>>>,
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u16) -> Result<Server, io::Error> {
        Ok(Server {
            clients: Arc::new(Mutex::new(HashMap::new())),
            listener: TcpListener::bind(format!("127.0.0.1:{}", port))?,
        })
    }

    pub fn run(&self) {
        let (sender, receiver) = mpsc::channel::<String>();
    }

    fn handle_events(&self) {
        loop {}
    }

    fn listen(&self) {
        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => self.accept(stream, addr),
                Err(err) => eprintln!("ERROR: could not accept connection: {}", err),
            }
        }
    }

    fn accept(&self, stream: TcpStream, addr: SocketAddr) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(addr, Client::new(stream));
    }
}

fn main() {
    let server = Server::new(1337).unwrap();
    server.run();
}
