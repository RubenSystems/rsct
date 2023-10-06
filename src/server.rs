use crate::{
    client::{Client},
    packet::PacketContainer,
    recieve::{recieve_once, RecieveError},
    transmit,
};
use tokio::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
}

// Internals
impl Server {
    pub async fn new(ip: &str, port: &str) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server {
            socket: socket,
        }
    }
}

// Transmitting
impl Server {
    pub async fn transmit(&self, data: &[u8], dest: &Client) {
        transmit::transmit(data, &self.socket, &dest.address()).await;
    }
}

impl Server {
    pub async fn recieve_once(&self, timeout_millis: u64) -> Result<PacketContainer, RecieveError> {
        recieve_once(&self.socket, timeout_millis).await
    }
}
