use std::net::SocketAddr;

use laminar::Socket;

use common::lobby_address;

pub struct Relay {
    pub socket: Socket,
    pub lobby_addr: SocketAddr,
    pub addr: SocketAddr,
    pub players: Option<Vec<SocketAddr>>,
}

impl Relay {
    pub fn new() -> Self {
        info!("Relay initialized");
        let socket = Socket::bind_any().unwrap();
        let addr = socket.local_addr().unwrap();
        let lobby_addr = lobby_address();
        Relay {
            socket,
            lobby_addr,
            addr,
            players: None,
        }
    }
}
