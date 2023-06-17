use std::net::SocketAddr;
use serde_derive::{Deserialize, Serialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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
