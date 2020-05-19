use std::io;
use core::events::{ClientEvent, ServerEvent};
use std::net::UdpSocket;

const SERVER_HOST: str = "localhost";
const SERVER_PORT: str = "25566";

pub struct NetworkHandler {
    socket: UdpSocket,
}

impl NetworkHandler {
    pub fn new() -> io::Result<Self> {
        self {
            socket: UdpSocket::bind("0.0.0.0:0")?
                .connect(format!("{}:{}", SERVER_HOST, SERVER_PORT))?,
        }
    }

    pub fn send(event: ClientEvent) {
        let buffer = bincode::serialize(&event).unwrap();
        self.socket.send(&buffer);
    }

    pub fn 
}
