#[macro_use]
extern crate log;

use std::{time::Instant, io::stdin};

use common::{lobby_address, client_address, DataType};
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

        socket.send(Packet::reliable_unordered(
            server,
            line.clone().into_bytes(),
        )).unwrap();

        socket.manual_poll(Instant::now());

        if line == "Bye!" {
            break;
        }

        match socket.recv() {
            Some(SocketEvent::Packet(packet)) => {
                if packet.addr() == server {
                    println!("Server sent: {}", String::from_utf8_lossy(packet.payload()));
                } else {
                    println!("Unknown sender.");
                }
            }
            Some(SocketEvent::Timeout(_)) => {}
            _ => println!("Silence.."),
        }
    }
}
