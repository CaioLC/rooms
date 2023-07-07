#[macro_use]
extern crate log;
use bincode::{deserialize, serialize};
use laminar::{Packet, Socket, SocketEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::net::SocketAddr;
use std::{thread, time::Instant};

use common::{LobbyEvents};

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

    let mut relay = Relay::new();
    let (sender, receiver) = (
        relay.socket.get_packet_sender(),
        relay.socket.get_event_receiver(),
    );
    let _thread = thread::spawn(move || relay.socket.start_polling());

    let init_event = LobbyEvents::RelayInitialized { relay_id: 0, relay_addr: relay.addr };
    sender.send(Packet::reliable_unordered(relay.lobby_addr, serialize(&init_event).unwrap()));
    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(_) => {},
                SocketEvent::Connect(_) => {},
                SocketEvent::Timeout(_) => {},
                SocketEvent::Disconnect(_) => {},
            }
        }
    }
}
