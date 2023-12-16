use crate::{
    client::{Client},
    packet::PacketContainer,
    recieve::{recieve_once, RecieveError},
    transmit,
};
use std::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
}

// Internals
impl Server {
    pub fn new(ip: &str, port: &str) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).unwrap();

        Server {
            socket: socket,
        }
    }
}

// Transmitting
impl Server {
    pub fn transmit(&self, data: &[u8], dest: &Client) {
        transmit::transmit(data, &self.socket, &dest.address());
    }
}

impl Server {
    pub fn recieve_once(&self) -> Result<PacketContainer, RecieveError> {
        recieve_once(&self.socket)
    }
}
