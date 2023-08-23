use crate::packet::{Packet, PacketContainer, MAX_DATA_SIZE, PACKET_HEADER_SIZE};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn transmit(
    data: *const u8,
    data_size: usize,
    socket: &UdpSocket,
    destination: &SocketAddr,
) {
    let mut pack = PacketContainer::new((data_size / MAX_DATA_SIZE) as u8);

    for offset in (0..data_size).step_by(MAX_DATA_SIZE) {
        let size: usize = (data_size - offset).min(MAX_DATA_SIZE);
        let data_slice: &[u8] = unsafe { std::slice::from_raw_parts(data, size) };
        pack.copy_data_to(data_slice);

        transmit_packet(&pack, socket, destination).await;

        pack.next();
    }
}

async fn transmit_packet(
    packet_container: &PacketContainer,
    socket: &UdpSocket,
    destination: &SocketAddr,
) {
    let message: *const u8 = &packet_container.packet as *const _ as *const u8;
    let struct_bytes =
        unsafe { std::slice::from_raw_parts(message, std::mem::size_of::<Packet>()) };

    socket
        .send_to(
            &struct_bytes[..(PACKET_HEADER_SIZE + packet_container.packet_data_size)],
            destination,
        )
        .await
        .unwrap();
}
