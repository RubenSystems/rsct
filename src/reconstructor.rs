
use std::collections::BTreeMap;
use crate::packet::{Packet, PacketContainer};
use crate::client::Client;

const PACKET_TTL: u8 = 3;

struct PacketStore {
	packets: Vec<PacketContainer>,
	from: Option<Client>,
	packet_id: u8,
	required_size: u8,
	ttl: u8
}

impl PacketStore {
	fn new(from: PacketContainer) -> PacketStore {
		PacketStore {
			packets: vec![from],
			from: from.from,
			packet_id: from.packet.header.client_tied_id,
			required_size: from.packet.header.total,
			ttl: PACKET_TTL
		}
	}
}

struct Reconstructor {
	store: 
}