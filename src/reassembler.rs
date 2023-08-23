use crate::client::Client;
use crate::packet::{PacketContainer, MAX_DATA_SIZE};
use lru::LruCache;
use std::num::NonZeroUsize;


const REASSEMBLER_SIZE : usize = 10;

struct PacketStore {
    data: Vec<u8>,
    from: Option<Client>,
    packet_id: u8,
    required_size: u8,
    packet_count: usize,
    copied_bytes: usize,
}

impl PacketStore {
    pub fn new(from: PacketContainer) -> PacketStore {
        let mut store = PacketStore {
            data: vec![0; from.packet.header.total as usize * MAX_DATA_SIZE],
            from: from.from,
            packet_id: from.packet.header.client_tied_id,
            required_size: from.packet.header.total,
            packet_count: 1,
            copied_bytes: from.packet_data_size,
        };

        store.data[(from.packet.header.index as usize * MAX_DATA_SIZE)..MAX_DATA_SIZE]
            .copy_from_slice(&from.packet.data);
        store
    }

    pub fn add(&mut self, packet: PacketContainer) {
        self.packet_count += 1;
        self.copied_bytes += packet.packet_data_size;
        self.data[(packet.packet.header.index as usize * MAX_DATA_SIZE)..MAX_DATA_SIZE]
            .copy_from_slice(&packet.packet.data);
    }

    pub fn is_complete(&self) -> bool {
        self.packet_count >= self.required_size as usize
    }
}

struct Reassembler {
    store: LruCache<String, PacketStore>,
}

impl Reassembler {

    fn new() -> Reassembler {
        Reassembler { 
            store: LruCache::new(NonZeroUsize::new(REASSEMBLER_SIZE).unwrap())
        }
    }

    fn add(&mut self, packet: PacketContainer) {
        let res = self.store.get_mut(&packet.informal_id());
        if let Some(packet_store) = res {
            packet_store.add(packet);
        } else {
            self.store.push(packet.informal_id(), PacketStore::new(packet));
        }
    }

}