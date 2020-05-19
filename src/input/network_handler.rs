use core::events::{ClientEvent, ServerEvent};
use serde::export::Err as SerdeErr;
use serde::export::Ok as SerdeOk;
use std::io;
use std::net::UdpSocket;

const MAX_MESSAGE_SIZE: usize = 65535;

const SERVER_HOST: &'static str = "localhost";
const SERVER_PORT: &'static str = "25565";

pub struct NetworkHandler {
    socket: UdpSocket,
}

impl NetworkHandler {
    pub fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(true).unwrap();
        socket.connect(format!("{}:{}", SERVER_HOST, SERVER_PORT))?;

        Ok(Self { socket })
    }

    pub fn send(&self, event: ClientEvent) {
        let buffer = bincode::serialize(&event).unwrap();

        self.socket.send(&buffer);
    }

    pub fn process(&self) -> io::Result<Vec<ServerEvent>> {
        let mut events = Vec::new();

        loop {
            let mut data = [0; MAX_MESSAGE_SIZE];
            if self.socket.recv(&mut data).is_err() {
                break;
            }

            match bincode::deserialize(&data) {
                SerdeOk(event) => events.push(event),
                SerdeErr(err) => println!("{}", err),
            }
        }

        Ok(events)
    }
}
