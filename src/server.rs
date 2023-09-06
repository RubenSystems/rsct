use crate::{client, reassembler, recieve::recieve_once, transmit};
use tokio::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
    reassembler: reassembler::Reassembler,
}

// Internals
impl Server {
    pub async fn new(ip: &str, port: &str) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server {
            socket,
            reassembler: reassembler::Reassembler::new(),
        }
    }
}

// Transmitting
impl Server {
    pub async fn transmit(&self, data: &[u8], dest: &client::Client) {
        transmit::transmit(data, &self.socket, &dest.address()).await;
    }
}

impl Server {
    pub async fn recieve_once(&mut self) -> reassembler::ReassemblerResult {
        let packet = recieve_once(&self.socket).await;
        self.reassembler.add(packet)
    }
}
