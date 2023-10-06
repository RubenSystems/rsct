use crate::client::Client;
use crate::packet::{Packet, PacketContainer, PACKET_HEADER_SIZE, PACKET_SIZE};
use tokio::net::UdpSocket;
use tokio::time::Duration;


pub enum RecieveError {
    Timeout, 
    RecieveFromError(String)
}

// timeout in milliseconds
pub async fn recieve_once(socket: &UdpSocket, timeout: u64) -> Result<PacketContainer, RecieveError> {
    let mut recieve_buffer = [0u8; PACKET_SIZE];

    let (len, addr) = tokio::select! {
        recieved = async {
            
            let (data_len, client) = socket.recv_from(&mut recieve_buffer).await.map_err(|e| RecieveError::RecieveFromError(e.to_string()))?;
            Ok((data_len, client))
        } => recieved,
        timeout = async {
            tokio::time::sleep(Duration::from_millis(timeout)).await;
            Err(RecieveError::Timeout)
        } => timeout
    }?;


    

    let pack: Packet = unsafe { std::ptr::read(recieve_buffer.as_ptr() as *const Packet) };

    Ok(PacketContainer {
        packet: pack,
        packet_data_size: len - PACKET_HEADER_SIZE,
        from: Some(Client::from_address(addr)),
    })
}
