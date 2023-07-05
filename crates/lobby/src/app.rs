use std::{
    net::SocketAddr,
    thread::{self, sleep},
    time::Duration,
};

use common::lobby_address;
use laminar::{Packet, Socket, SocketEvent};

pub struct Lobby {
    pub socket: Socket,
    pub server_rooms: Option<Vec<SocketAddr>>,
}

impl Lobby {
    pub fn new() -> Self {
        info!("Lobby started");
        let socket = Socket::bind(lobby_address()).unwrap();
        Lobby {
            socket,
            server_rooms: None,
        }
    }
}
