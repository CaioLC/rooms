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
                GameState::Lobby => self.lobby_loop(),
                GameState::AwaitingRoom => self.await_room_loop(),
                GameState::Room(_, _) => self.room_loop(),
                GameState::Quit => self.quit(),
            }
        }
    }

    fn lobby_loop(&mut self) {
        // check messages
        while let Ok(event) = self.receiver.recv_timeout(self.timeout) {
            if let SocketEvent::Packet(packet) = event {
                if packet.addr() == self.lobby_addr {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    match event {
                        LobbyEvents::Message(msg) => {
                            println!("server sent: {msg}")
                        },
                        _ => {}
                    }
                } else {
                    println!("Unknown sender.");
                }
            }
        }

        // parse cli
        // TODO: this needs to be async.
        let stdin = stdin();
        let mut s_buffer = String::new();
        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
        let message = LobbyEvents::from_string(&line);

        // send message
        self.sender
            .send(Packet::reliable_unordered(
                self.lobby_addr,
                serialize(&message).unwrap(),
            ))
            .unwrap();
        
        // if command is new game, update GameState.
        if message == LobbyEvents::ClientCreateGame {
            println!("Awaiting game creation. Send 'cancel' to abort.");
            self.game_state = GameState::AwaitingRoom;
        }
    }

    fn await_room_loop(&mut self) {
        let stdin = stdin();
        let mut s_buffer = String::new();
        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
        if line == "cancel".to_string() {
            self.game_state = GameState::Lobby;
            println!("returning to Lobby");
            return
        }
        while let Ok(event) = self.receiver.recv_timeout(self.timeout) {
            if let SocketEvent::Packet(packet) = event {
                if packet.addr() == self.lobby_addr {
                    let payload = packet.payload();
                    let event: LobbyEvents = deserialize(payload).unwrap();
                    match event {
                        LobbyEvents::RelayInitialized { relay_id, relay_addr } => update_game_state(self, relay_id, relay_addr),
                        LobbyEvents::Message(msg) => info!("server sent: {msg}"),
                        _ => {}
                    }
                } else {
                    println!("Unknown sender.");
                }
            }
        }
    }

    fn room_loop(&mut self) {
        todo!()
    }

    fn quit(&mut self) {
        todo!()
    }
}

fn update_game_state(arg: &mut Client, relay_id: u32, relay_addr: SocketAddr) {
    todo!()
}