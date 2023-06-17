//! This module provides an simple client, server examples with communication over udp.
//! 1. setting up server to receive data.
//! 2. setting up client to send data.
//! 3. serialize data to send and deserialize when received.
#[macro_use]
extern crate log;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::{time::Instant, thread};
use bincode::{deserialize, serialize};
use laminar::{Packet, Socket, SocketEvent};

use common::{lobby_address, DataType};

#[allow(unused_must_use)]
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("A logger was already initialized");
    // This will run an simple example with client and server communicating.
    info!("Lobby listening...");
    let mut socket = Socket::bind(lobby_address()).unwrap();
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();

                    if msg == b"Bye!" {
                        break;
                    }

                    let msg = String::from_utf8_lossy(msg);
                    let ip = packet.addr().ip();

                    println!("Received {:?} from {:?}", msg, ip);

                    sender
                        .send(Packet::reliable_unordered(
                            packet.addr(),
                            "Copy that!".as_bytes().to_vec(),
                        ))
                        .expect("This should send");
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }
}
