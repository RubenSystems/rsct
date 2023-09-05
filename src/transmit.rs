use crate::packet::{Packet, PacketContainer, MAX_DATA_SIZE, PACKET_HEADER_SIZE};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn transmit(data: &[u8], socket: &UdpSocket, destination: &SocketAddr) {
    let mut pack = PacketContainer::new((data.len() / MAX_DATA_SIZE) as u16 + 1);

    for offset in (0..data.len()).step_by(MAX_DATA_SIZE) {
        let size: usize = (data.len() - offset).min(MAX_DATA_SIZE);
        pack.copy_data_to(&data[offset..(offset + size)]);

        transmit_packet(&pack, socket, destination).await;

        pack.next();
    }
}

async fn transmit_packet(
    packet_container: &PacketContainer,
    socket: &UdpSocket,
    destination: &SocketAddr,
) {
    let struct_bytes: &[u8] = unsafe {
        let packet_ref = &packet_container.packet;
        let message: *const Packet = packet_ref;
        std::slice::from_raw_parts(message as *const u8, std::mem::size_of::<Packet>())
    };

    socket
        .send_to(
            &struct_bytes[..(PACKET_HEADER_SIZE + packet_container.packet_data_size)],
            destination,
        )
        .await
        .unwrap();
}
