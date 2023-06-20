#[macro_use]
extern crate log;

use std::net::SocketAddr;
use serde_derive::{Deserialize, Serialize};

/// GAME ///
pub enum GameState {
    Lobby,
    Room(usize, usize), // relay_id, game_id
    Quit,
}


/// NETWORKING ///
/// The socket address of where the server is located.
const LOBBY_ADDR: &str = "127.0.0.1:12345";
// The client address from where the data is sent.
const CLIENT_ADDR: &str = "127.0.0.1:12346";

pub fn client_address() -> SocketAddr {
    CLIENT_ADDR.parse().unwrap()
}

pub fn lobby_address() -> SocketAddr {
    LOBBY_ADDR.parse().unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LobbyEvents {
    ClientRequestGameList,
    ClientRequestHelp,
    ClientCreateGame,
    ClientJoinGame{relay_id: u32, game_id: u32},
    RelayConnected,
    RelayDisconnected,
    KeepAlive,
    Message(String),
}

impl LobbyEvents {
    pub fn from_string(cmd: &String) -> LobbyEvents {
        if cmd == "lobby -h" || cmd == "lobby --help" {
            return LobbyEvents::ClientRequestHelp;
        }
        if cmd == "lobby -l" || cmd == "lobby --list" {
            return LobbyEvents::ClientRequestGameList;
        }
        if cmd == "lobby -n" || cmd == "lobby --newgame" {
            return LobbyEvents::ClientCreateGame;
        }
        if cmd.starts_with("lobby -j") || cmd.starts_with("lobby --join") {
            info!("Requested join");
            let mut parse_command = cmd.split_whitespace();
            let _ = parse_command.next().unwrap();
            let _ = parse_command.next().unwrap();
            let relay_id = parse_command.next();
            let game_id = parse_command.next();
            let event = match (relay_id, game_id) {
                (Some(relay), Some(game)) => {
                    let relay_id = relay.trim().parse().unwrap();
                    let game_id = game.trim().parse().unwrap();
                    LobbyEvents::ClientJoinGame { relay_id, game_id }
                }
                _ => LobbyEvents::Message(format!("ERR: failed to parse relay and game ID from command: {cmd}"))
            };
            return event
        }
        return LobbyEvents::Message(cmd.into());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    Coords {
        longitude: f32,
        latitude: f32,
        altitude: f32,
    },
    Text {
        string: String,
    },
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
