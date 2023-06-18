#[macro_use]
extern crate log;

use std::{time::Instant, io::stdin};

use common::{lobby_address, client_address, DataType, LobbyEvents};
use bincode::{deserialize, serialize};
use laminar::{Socket, Packet, SocketEvent};
fn main() {
    /*  setup our `Client` and send some test data. */
    let mut socket = Socket::bind(client_address()).unwrap();
    info!("Connected on {}", client_address());

    let server = lobby_address();

    println!("Type a message and press Enter to send. Send `Bye!` to quit.");

    let stdin = stdin();
    let mut s_buffer = String::new();

    loop {
        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
        
        let message = LobbyEvents::from_string(&line);
        socket.send(Packet::reliable_unordered(
            server,
            serialize(&message).unwrap()
        )).unwrap();
        
        if &line == "Bye!" {
                    break;
                }

        socket.manual_poll(Instant::now());        
        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    match event {
                        LobbyEvents::Message(msg) => {
                            println!("Server sent: {msg}");
                        }
                        _ => {}
                    }
                } else {
                    println!("Unknown sender.");
                }
            }
            _ => {}
        }
    }
}
