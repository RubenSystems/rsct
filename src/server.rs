use crate::allocators::allocator::Allocator;
use crate::{
    client::{self, Client},
    reassembler,
    recieve::recieve_once,
    transmit,
};

use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Server<T: Allocator> {
    socket: Arc<UdpSocket>,
    reassembler: reassembler::Reassembler<T>,
}

// Internals
impl<T: Allocator> Server<T> {
    pub async fn new(ip: &str, port: &str, allocator: T) -> Server<T> {
        let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

        Server::<T> {
            socket: Arc::new(socket),
            reassembler: reassembler::Reassembler::<T>::new(allocator),
        }
    }
}

// Transmitting
impl<T: Allocator> Server<T> {
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

impl<T: Allocator> Server<T> {
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
