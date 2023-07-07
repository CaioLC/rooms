//! This module provides an simple client, server examples with communication over udp.
//! 1. setting up server to receive data.
//! 2. setting up client to send data.
//! 3. serialize data to send and deserialize when received.
#[macro_use]
extern crate log;
use bincode::{deserialize, serialize};
use laminar::{Packet, Socket, SocketEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::{thread, time::Instant, process::Command};

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
    let (sender, receiver) = (
        lobby.socket.get_packet_sender(),
        lobby.socket.get_event_receiver(),
    );
    let _thread = thread::spawn(move || lobby.socket.start_polling());

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    match event {
                        LobbyEvents::ClientRequestHelp => {
                            let response = serialize(&LobbyEvents::Message(
                                r#"Commands available: 
lobby --help | lobby -h: display this help message.
lobby --list | lobby -l: list games available.
lobby --newgame | lobby -n: create new game.
lobby --quit | lobby -q to quit."#
                                    .to_string(),
                            ))
                            .unwrap();
                            sender
                                .send(Packet::reliable_unordered(packet.addr(), response))
                                .expect("This should send");
                        }
                        LobbyEvents::ClientRequestGameList => {
                            println!("TODO: Show game list")
                        }
                        LobbyEvents::ClientCreateGame => {
                            let id = match &lobby.server_rooms {
                                Some(v) => v.len(),
                                None => 0,
                            }; 
                            
                            // NOTE: this will run relays in the same machine of the lobby.
                            // Ideally we can spawn relays anywhere and have the lobby keep address records.
                            let output = if cfg!(target_os = "windows") {
                                Command::new("cmd")
                                        .args(["/C", &format!("cargo run -p relay {id}")])
                                        .output()
                                        .expect("failed to execute process")
                            } else {
                                Command::new("sh")
                                        .arg("-c")
                                        .arg(format!("cargo run -p relay {id}"))
                                        .output()
                                        .expect("failed to execute process")
                            };
                            println!("{:?}", output);
                        }
                        LobbyEvents::ClientJoinGame { relay_id, game_id } => {
                            println!("TODO: joining game: {relay_id}--{game_id}")
                        }
                        LobbyEvents::RelayInitialized {relay_id, relay_addr} => {
                            println!("Relay live at {relay_addr} with id {relay_id}");
                        },
                        LobbyEvents::RelayDisconnected => todo!(),
                        LobbyEvents::KeepAlive => {}
                        LobbyEvents::Message(msg) => {
                            dbg!(&msg);
                            if msg == "Bye!" {
                                break;
                            }
                            let ip = packet.addr().ip();
                            info!("Received {:?} from {:?}", msg, ip);

                            let response =
                                serialize(&LobbyEvents::Message("Copy that!".to_string())).unwrap();
                            sender
                                .send(Packet::reliable_unordered(packet.addr(), response))
                                .expect("This should send");
                        }
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
