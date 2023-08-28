use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

pub struct Client {
    _address: SocketAddr,
}

impl Client {
    pub fn from_string(address: String) -> Client {
        Client {
            _address: address.parse().unwrap(),
        }
    }

    pub fn from_address(address: SocketAddr) -> Client {
        Client { _address: address }
    }

    pub fn address(&self) -> SocketAddr {
        self._address
    }

    pub fn ip_string(&self) -> String {
        self._address.ip().to_string()
    }

    pub fn port_string(&self) -> String {
        self._address.port().to_string()
    }
}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.port_string().hash(state);
        self.ip_string().hash(state);
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.port_string() == other.port_string() && self.ip_string() == other.ip_string()
    }
}
