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

use common::{lobby_address, DataType, LobbyEvents};

mod app;
use app::Lobby;

#[allow(unused_must_use)]
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("A logger was already initialized");
    // This will run an simple example with client and server communicating.
    info!("Lobby listening...");
    let mut lobby = Lobby::new();
    let (sender, receiver) = (lobby.socket.get_packet_sender(), lobby.socket.get_event_receiver());
    let _thread = thread::spawn(move || lobby.socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    match event {
                        LobbyEvents::ClientRequestHelp => {
                            let response = serialize(
                                &LobbyEvents::Message(
                                    format!("Commands available: \nlobby --help: display this help message.\nlobby --list: list games available.\n lobby --newgame: create new game.\nWrite `Bye!` to quit."))
                            ).unwrap();
                            sender
                                .send(Packet::reliable_unordered(
                                    packet.addr(),
                                    response,
                                ))
                                .expect("This should send");
                        },
                        LobbyEvents::ClientRequestGameList => {println!("TODO: Show game list")}
                        LobbyEvents::ClientCreateGame => {println!("TODO: Create game")}
                        LobbyEvents::ClientJoinGame{relay_id, game_id} => {println!("TODO: joining game: {relay_id}--{game_id}")}
                        LobbyEvents::RelayConnected => todo!(),
                        LobbyEvents::RelayDisconnected => todo!(),
                        LobbyEvents::KeepAlive => {},
                        LobbyEvents::Message(msg) => {
                            if msg == "Bye!" {
                                break;
                            }
                            let ip = packet.addr().ip();
                            info!("Received {:?} from {:?}", msg, ip);

                            let response = serialize(&LobbyEvents::Message(format!("Copy that!"))).unwrap();
                            sender
                                .send(Packet::reliable_unordered(
                                    packet.addr(),
                                    response,
                                ))
                                .expect("This should send");
                        },
                    }
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {address}");
                }
                SocketEvent::Connect(socket_addr) => {
                    info!("Client connected to Lobby: {socket_addr}")
                }
                SocketEvent::Disconnect(socket_addr) => {
                    info!("Client disconnected from Lobby: {socket_addr}.")
                }
            }
        }
    }
}
