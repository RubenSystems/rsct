use crate::packet::{Packet, PacketContainer, MAX_DATA_SIZE, PACKET_HEADER_SIZE, generate_client_tied_uid};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;


pub async fn transmit(data: &[u8], socket: &UdpSocket, destination: &SocketAddr) {
    let mut pack = PacketContainer::new((data.len() / MAX_DATA_SIZE) as u16 + 1, 0);

    for offset in (0..data.len()).step_by(MAX_DATA_SIZE) {
        let size: usize = (data.len() - offset).min(MAX_DATA_SIZE);
        pack.copy_data_to(&data[offset..(offset + size)]);

        transmit_packet(&pack, socket, destination).await;

        pack.next();
    }
}

pub async fn transmit_concurrently(
    data: &[u8],
    socket: Arc<UdpSocket>,
    destination: Arc<SocketAddr>,
    runtime: &tokio::runtime::Runtime,
) {
    let mut handles = Vec::new();
    let total_packet_count = (data.len() / MAX_DATA_SIZE) as u16 + 1;
    let client_tied_id = generate_client_tied_uid();

    for (index, offset) in (0..data.len()).step_by(MAX_DATA_SIZE).enumerate() {
        let size: usize = (data.len() - offset).min(MAX_DATA_SIZE);
        let mut data_slice = [0u8; MAX_DATA_SIZE];

        data_slice[..size].copy_from_slice(&data[offset..(offset + size)]);
        let mut pack = PacketContainer::new_with_fixed_client_uid(total_packet_count, index as u16, client_tied_id);
        let sock_ref = Arc::clone(&socket);
        let sock_dest = Arc::clone(&destination);
        let handle = runtime.spawn(async move {
            pack.move_data_to(data_slice);
            transmit_packet(&pack, &sock_ref, &sock_dest).await;
        });

        handles.push(handle);
    }

    
    for handle in handles {
        handle.await.expect("PACKET SEND TASK PANICED");
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
