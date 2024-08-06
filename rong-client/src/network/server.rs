use std::io::ErrorKind;
use std::net::UdpSocket;

const SERVER_ADDR: &str = "127.0.0.1:2906";

pub struct Server {
    socket: UdpSocket,
}

impl Server {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind UDP socket");

        socket
            .set_nonblocking(true)
            .expect("Could not set UDP socket to non-blocking");

        socket
            .connect(SERVER_ADDR)
            .expect("Could not bind to server address");

        Server { socket }
    }

    pub fn send_connect(&self) {
        self.socket
            .send(b"CONNECT")
            .expect("Failed to send connect message");
    }

    pub fn receive(&self) -> Result<Option<String>, std::io::Error> {
        let mut buf = [0; 1024];
        match self.socket.recv(&mut buf) {
            Ok(amt) => {
                let received = std::str::from_utf8(&buf[..amt]).unwrap();
                Ok(Some(received.to_string()))
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                // No data available right now, not an error
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    pub fn send_key_press(&self, player_id: String, key: &str) {
        // message format: "<player_id> <key>"
        let message = format!("{} {}", player_id, key);
        self.socket
            .send(message.as_bytes())
            .expect("Failed to send key press");
    }
}
