use std::{thread::{sleep, self}, time::Duration, net::SocketAddr};

use laminar::{Socket, Packet, SocketEvent};
use common::lobby_address;

pub struct Lobby {
    pub socket: Socket,
    pub server_rooms: Option<Vec<SocketAddr>>
}

impl Lobby {
    pub fn new() -> Self {
        info!("Lobby started");
        let address = Socket::bind(lobby_address()).unwrap();
        Lobby {
            socket: address,
            server_rooms: None,
        }
    }
}