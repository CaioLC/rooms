use std::{net::SocketAddr, io::stdin, time::Instant, thread};

use bincode::{serialize, deserialize};
use common::{GameState, client_address, lobby_address, LobbyEvents};
use laminar::{Socket, Packet, SocketEvent};
use crossbeam::channel::{Sender, Receiver};

pub struct Client {
    pub game_state: GameState,
    pub sender: Sender<Packet>,
    pub receiver: Receiver<SocketEvent>,
    pub lobby_addr: SocketAddr,
    pub relay_addr: Option<SocketAddr>,
}

impl Client {
    pub fn new(sender: Sender<Packet>, receiver: Receiver<SocketEvent>) -> Self {
        info!("Client started");
        let lobby_addr = lobby_address();
        Client { game_state: GameState::Lobby, sender, receiver, lobby_addr, relay_addr: None }
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
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    if packet.addr() == self.lobby_addr {
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
        let stdin = stdin();
        let mut s_buffer = String::new();

        s_buffer.clear();
        stdin.read_line(&mut s_buffer).unwrap();
        let line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
        
        let message = LobbyEvents::from_string(&line);
        dbg!(&message);
        self.sender.send(Packet::reliable_unordered(
            self.lobby_addr,
            serialize(&message).unwrap()
        )).unwrap();


    }

    fn room_interaction(&mut self) {
        todo!()
    }

    fn quit(&mut self) {
        todo!()
    }
}