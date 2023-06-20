#[macro_use]
extern crate log;

use std::{time::Instant, io::stdin, thread};

use common::{lobby_address, client_address, DataType, LobbyEvents};
use bincode::{deserialize, serialize};
use laminar::{Socket, Packet, SocketEvent};

mod app;
use app::Client;

fn main() {
    // bind to socket, get handles to sender and receiver and send pooling to another thread.
    let mut socket = Socket::bind(client_address()).unwrap();
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
    let _thread = thread::spawn(move || socket.start_polling());
    
    /* setup client */
    let mut client = Client::new(sender, receiver);
    println!("Type a message and press Enter to send. Send `Bye!` to quit.");
    client.run();
}
