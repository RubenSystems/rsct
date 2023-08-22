use crate::packet::{PacketContainer, Packet, PACKET_HEADER_SIZE, PACKET_SIZE};
use tokio::net::UdpSocket;
use crate::client::Client;



pub async fn recieve_once(socket: &UdpSocket) -> PacketContainer {
	let mut recieve_buffer = [0u8; PACKET_SIZE];
	let (len, addr) = socket.recv_from(&mut recieve_buffer).await.unwrap();

	let pack: Packet = unsafe {
		std::ptr::read(recieve_buffer.as_ptr() as *const Packet)
	};

	PacketContainer {
		packet: pack, 
		packet_data_size: len - PACKET_HEADER_SIZE,
		from: Some(Client::from_address(addr))
	}
}