use crate::client::Client;
use crate::packet::{Packet, PacketContainer, PACKET_HEADER_SIZE, PACKET_SIZE};
use tokio::net::UdpSocket;


pub enum RecieveError {
    RecieveFromError(String)
}

// timeout in milliseconds
pub async fn recieve_once(socket: &UdpSocket) -> Result<PacketContainer, RecieveError> {
    let mut recieve_buffer = [0u8; PACKET_SIZE];

    let (len, addr) = socket.recv_from(&mut recieve_buffer).await.map_err(|e| RecieveError::RecieveFromError(e.to_string()))?;

    let pack: Packet = unsafe { std::ptr::read(recieve_buffer.as_ptr() as *const Packet) };

    Ok(PacketContainer {
        packet: pack,
        packet_data_size: len - PACKET_HEADER_SIZE,
        from: Some(Client::from_address(addr)),
    })
}
