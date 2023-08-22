use crate::{recieve::recieve_once, transmit, packet, client};
use tokio::net::UdpSocket;
use std::collections::BTreeSet;

pub struct Server { 
	socket: UdpSocket
}

// Internals
impl Server {
	pub async fn new(ip: &str, port: &str) -> Server {
		let socket = UdpSocket::bind(format!("{}:{}", ip, port)).await.unwrap();

		Server {
			socket
		}
	}
}

// Transmitting
impl Server {
	pub async fn transmit(&self, data: *const u8, data_size: usize, dest: client::Client) {
		transmit::transmit(data, data_size, &self.socket, &&dest.address()).await;
	}
}

impl Server {
	pub async fn start_listener(&self) {
		loop {
			let packet = recieve_once(&self.socket).await;
			let from = packet.from.unwrap();
			println!("{} : {}", from.ip_string(), from.port_string());
		}
	}
}