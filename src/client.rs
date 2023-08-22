use std::net::SocketAddr;


pub struct Client {
	address: SocketAddr
}

impl Client {
	pub fn new(address: String) -> Client {
		Client {
			address: address.parse().unwrap()
		}
	}

	pub fn get_address(&self) -> SocketAddr {
		self.address
	}
}