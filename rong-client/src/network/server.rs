use bincode;
use rong_shared::error::ClientError;
use rong_shared::model::{
    ClientMessage, Movement, MovementPacket, NetworkPacket, PlayerId, ServerMessage,
};
use std::io::{ErrorKind, Read, Write};
use std::net::UdpSocket;
use std::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1:2906";

pub struct Server {
    socket: UdpSocket,
    sequence_number: u32,
    pub player_id: Option<PlayerId>,
}

impl Server {
    pub fn new() -> Result<Self, ClientError> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        socket.set_nonblocking(true)?;
        socket.connect(SERVER_ADDR)?;

        Ok(Server {
            socket,
            sequence_number: 0,
            player_id: None,
        })
    }

    pub fn send_connect(&mut self) -> Result<(), ClientError> {
        let message: ClientMessage = ClientMessage::Connect();
        self.send_packet(message)?;

        // Wait for response with a timeout
        let start_time = std::time::Instant::now();
        while start_time.elapsed() < Duration::from_secs(5) {
            match self.receive() {
                Ok(Some(ServerMessage::PlayerJoined(id))) => {
                    self.player_id = Some(id);

                    self.send_packet(ClientMessage::ConnectionAck())?;
                    println!("Connection to server, awknowledged");

                    return Ok(());
                }
                Ok(Some(msg)) => {
                    println!("Unexpected message: {:?}", msg);
                }
                Ok(None) => {
                    // No message received, continue waiting
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(e),
            }
        }
        Err(ClientError::Io("Connection timeout".to_string()))
    }

    pub fn receive(&mut self) -> Result<Option<ServerMessage>, ClientError> {
        let mut buf = [0; 1024];
        match self.socket.recv(&mut buf) {
            Ok(amt) => {
                let packet: NetworkPacket<ServerMessage> = bincode::deserialize(&buf[..amt])?;
                let message = packet.get_payload().clone();
                if let ServerMessage::PlayerJoined(id) = &message {
                    self.player_id = Some(*id);
                }
                Ok(Some(message))
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(ClientError::Io(e.to_string())),
        }
    }

    pub fn send_movement(&mut self, movement: Movement) -> Result<(), ClientError> {
        if let Some(player_id) = self.player_id {
            let movement_packet = MovementPacket::new(player_id, movement);
            let message = ClientMessage::MovementCommand(movement_packet);
            self.send_packet(message)
        } else {
            Err(ClientError::Io("Player ID not set".to_string()))
        }
    }

    fn send_packet(&mut self, message: ClientMessage) -> Result<(), ClientError> {
        self.sequence_number += 1;
        let packet = NetworkPacket::new(self.sequence_number, 0, message);
        let serialized = bincode::serialize(&packet)?;
        self.socket.send(&serialized)?;
        Ok(())
    }
}
