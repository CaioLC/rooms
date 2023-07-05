use std::time::Duration;
use std::{io::stdin, net::SocketAddr, thread, time::Instant};

use bincode::{deserialize, serialize};
use common::{client_address, lobby_address, GameState, LobbyEvents};
use crossbeam::channel::{Receiver, Sender};
use laminar::{Packet, Socket, SocketEvent};

pub struct Client {
    pub username: String,
    pub game_state: GameState,
    pub sender: Sender<Packet>,
    pub receiver: Receiver<SocketEvent>,
    pub lobby_addr: SocketAddr,
    pub relay_addr: Option<SocketAddr>,
    pub timeout: Duration,
}

impl Client {
    pub fn new(username: String, sender: Sender<Packet>, receiver: Receiver<SocketEvent>) -> Self {
        info!("Client started");
        let lobby_addr = lobby_address();
        Client {
            username,
            game_state: GameState::Lobby,
            sender,
            receiver,
            lobby_addr,
            relay_addr: None,
            timeout: Duration::from_millis(200),
        }
    }

    pub fn run(&mut self) {
        // TODO: do I need to sleep here?
        loop {
            match self.game_state {
                GameState::Lobby => self.lobby_interaction(),
                GameState::Room(_, _) => self.room_interaction(),
                GameState::Quit => self.quit(),
            }
        }
    }

    fn lobby_interaction(&mut self) {
        let stdin = stdin();
        let mut s_buffer = String::new();

        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");

        let message = LobbyEvents::from_string(&line);
        self.sender
            .send(Packet::reliable_unordered(
                self.lobby_addr,
                serialize(&message).unwrap(),
            ))
            .unwrap();

        while let Ok(event) = self.receiver.recv_timeout(self.timeout) {
            if let SocketEvent::Packet(packet) = event {
                if packet.addr() == self.lobby_addr {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    if let LobbyEvents::Message(msg) = event {
                        println!("server sent: {msg}");
                    }
                } else {
                    println!("Unknown sender.");
                }
            }
        }
    }

    fn room_interaction(&mut self) {
        todo!()
    }

    fn quit(&mut self) {
        todo!()
    }
}
