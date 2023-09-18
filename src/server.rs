use crate::{
    allocator::SimpleAllocator,
    buffer_allocator::BufferAllocator,
    client::{self, Client},
    reassembler,
    recieve::recieve_once,
    transmit,
};
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Server {
    socket: Arc<UdpSocket>,
    reassembler: reassembler::Reassembler,
}

// Internals
impl Server {
    pub async fn new(ip: &str, port: &str) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server {
            socket: Arc::new(socket),
            reassembler: reassembler::Reassembler::new(Box::new(SimpleAllocator {})),
        }
    }

    pub async fn new_with_buffer_allocator(ip: &str, port: &str, max_size: usize) -> Server {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server {
            socket: Arc::new(socket),
            reassembler: reassembler::Reassembler::new(Box::new(BufferAllocator::new(max_size))),
        }
    }
}

// Transmitting
impl Server {
    pub async fn transmit(&self, data: &[u8], dest: &client::Client) {
        transmit::transmit(data, &self.socket, &dest.address()).await;
    }

    pub async fn transmit_concurrently(
        &self,
        data: &[u8],
        dest: &client::Client,
        runtime: &tokio::runtime::Runtime,
    ) {
        transmit::transmit_concurrently(
            data,
            Arc::clone(&self.socket),
            Arc::new(dest.address()),
            runtime,
        )
        .await;
    }
}

impl Server {
    pub async fn recieve_once(&mut self) -> (Option<Client>, Vec<u8>) {
        loop {
            let packet = recieve_once(&self.socket).await;
            match self.reassembler.add(packet) {
                reassembler::ReassemblerResult::Complete(cli, dat) => return (cli, dat),
                _ => continue,
            }
        }
    }
}
