use crate::client::Client;
use crate::packet::{PacketContainer, MAX_DATA_SIZE};
use lru::LruCache;
use std::num::NonZeroUsize;

const REASSEMBLER_SIZE: usize = 3;

struct PacketStore {
    data: Vec<u8>,
    from: Option<Client>,
    required_size: u16,
    packet_count: usize,
    copied_bytes: usize,
}

impl PacketStore {
    pub fn new(from: PacketContainer) -> PacketStore {
        let mut store = PacketStore {
            data: vec![0; from.packet.header.total as usize * MAX_DATA_SIZE],
            from: from.from,
            required_size: from.packet.header.total,
            packet_count: 1,
            copied_bytes: from.packet_data_size,
        };
        let offset = from.packet.header.index as usize * MAX_DATA_SIZE;
        store.data[(offset)..(offset + MAX_DATA_SIZE)].copy_from_slice(&from.packet.data);
        store
    }

    pub fn add(&mut self, packet: PacketContainer) {
        self.packet_count += 1;
        self.copied_bytes += packet.packet_data_size;
        let offset = packet.packet.header.index as usize * MAX_DATA_SIZE;
        self.data[offset..(offset + MAX_DATA_SIZE)].copy_from_slice(&packet.packet.data);
    }

    pub fn is_complete(&self) -> bool {
        self.packet_count >= self.required_size as usize
    }
}

pub struct Reassembler {
    store: LruCache<String, PacketStore>,
}

pub enum ReassemblerResult {
    NotComplete,
    Complete(Option<Client>, Vec<u8>),
}

impl Reassembler {
    pub fn new() -> Reassembler {
        Reassembler {
            store: LruCache::new(NonZeroUsize::new(REASSEMBLER_SIZE).unwrap()),
        }
    }

    pub fn add(&mut self, packet: PacketContainer) -> ReassemblerResult {
        let id = packet.informal_id();

        let packet_store = if let Some(pkt_store) = self.store.pop(&id) {
            let mut pkt_store = pkt_store;
            pkt_store.add(packet);
            pkt_store
        } else {
            PacketStore::new(packet)
        };

        if packet_store.is_complete() {
            self.store.clear();
            ReassemblerResult::Complete(
                packet_store.from,
                packet_store.data[..packet_store.copied_bytes].to_vec(),
                
            )
        } else {
            self.store.push(id, packet_store);
            ReassemblerResult::NotComplete
        }
    }
}
