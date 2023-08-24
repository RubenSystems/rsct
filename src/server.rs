use crate::{client, recieve::recieve_once, transmit, reassembler};
use tokio::net::UdpSocket;

pub struct Server {
    socket: UdpSocket,
    reassembler: reassembler::Reassembler,
    listening: bool
}

// Internals
impl Server {
    pub async fn new(ip: &str, port: &str) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server { 
            socket,
            reassembler: reassembler::Reassembler::new(),
            listening: false
        }
    }
}

// Transmitting
impl Server {
    pub async fn transmit(&self, data: &[u8], dest: client::Client) {
        transmit::transmit(data, &self.socket, &dest.address()).await;
    }
}

impl Server {
    pub async fn start_listener(&mut self, callback: impl Fn(Option<client::Client>, Vec<u8>) -> ()) {
        self.listening = true;
        while self.listening {
            let packet = recieve_once(&self.socket).await;
            match self.reassembler.add(packet) {
                reassembler::ReassemblerResult::Complete(from, data) => {
                    callback(from, data);
                },
                reassembler::ReassemblerResult::NotComplete => continue
            }
        }
    }

    pub fn stop_listener(&mut self) {
        self.listening = false;
    }
}
