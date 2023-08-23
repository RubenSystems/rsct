use crate::client::Client;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU8, Ordering};

pub const MTU: usize = 1500;
static mut CURRENT_PACKET_INDEX: AtomicU8 = AtomicU8::new(0);

#[repr(C)]
pub struct PacketHeader {
    pub index: u8,
    pub client_tied_id: u8,
    pub total: u8,
}

pub const PACKET_HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();
pub const MAX_DATA_SIZE: usize = MTU - PACKET_HEADER_SIZE;

#[repr(C)]
pub struct Packet {
    pub header: PacketHeader,
    pub data: [u8; MAX_DATA_SIZE],
}

pub const PACKET_SIZE: usize = std::mem::size_of::<Packet>();

pub struct PacketContainer {
    pub packet: Packet,
    pub packet_data_size: usize,
    pub from: Option<Client>,
}

impl PacketContainer {
    pub fn new(total_packet_count: u8) -> PacketContainer {
        PacketContainer {
            packet: Packet {
                header: PacketHeader {
                    index: 0,
                    client_tied_id: unsafe { CURRENT_PACKET_INDEX.fetch_add(1, Ordering::SeqCst) },
                    total: total_packet_count,
                },
                data: [0u8; MAX_DATA_SIZE],
            },
            packet_data_size: 0,
            from: None,
        }
    }

    pub fn copy_data_to(&mut self, data: &[u8]) {
        self.packet_data_size = data.len();
        self.packet.data[..data.len()].copy_from_slice(data);
    }

    pub fn next(&mut self) {
        self.packet.header.index += 1;
    }

    pub fn informal_id(&self) -> String {
        if let Some(from) = &self.from {
            format!("{}:{}-{}", from.ip_string(), from.port_string(), self.packet.header.client_tied_id)
        } else {
            format!("-{}", self.packet.header.client_tied_id)
        }
    }
}

impl Hash for PacketContainer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(from) = &self.from {
            from.hash(state);
        }

        self.packet.header.client_tied_id.hash(state);
    }
}

impl Eq for PacketContainer {}

impl PartialEq for PacketContainer {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.packet.header.client_tied_id == other.packet.header.client_tied_id
    }
}
