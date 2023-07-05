#[macro_use]
extern crate log;
use bincode::{deserialize, serialize};
use laminar::{Packet, Socket, SocketEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::net::SocketAddr;
use std::{thread, time::Instant};

use common::{lobby_address, DataType, LobbyEvents};

mod app;
use app::Relay;

#[allow(unused_must_use)]
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("A logger was already initialized");
    // This will run an simple example with client and server communicating.
    info!("Relay initialized");

    let mut relay = Relay::new(lobby_address());
    let (sender, receiver) = (
        relay.socket.get_packet_sender(),
        relay.socket.get_event_receiver(),
    );
    let _thread = thread::spawn(move || relay.socket.start_polling());

    let init_event = LobbyEvents::RelayInitialized
    sender.send()
}
