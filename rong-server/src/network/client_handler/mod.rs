mod connection;

use bincode;
use rong_shared::model::{ClientMessage, NetworkPacket, ServerMessage};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};

pub struct ClientHandler {
    socket: Arc<UdpSocket>,
    clients: HashMap<SocketAddr, ClientInfo>,
    packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
    sequence: u32,
}

#[derive(Clone)]
pub struct ClientInfo {
    last_seen: Instant,
}

impl ClientHandler {
    pub async fn new(
        server_addr: SocketAddr,
        packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
    ) -> Result<Self, std::io::Error> {
        let socket = Arc::new(UdpSocket::bind(server_addr).await?);

        Ok(ClientHandler {
            socket,
            clients: HashMap::new(),
            packet_sender,
            sequence: 0,
        })
    }

    pub fn update_client(&mut self, client_addr: SocketAddr) {
        self.clients
            .entry(client_addr)
            .and_modify(|client| {
                client.last_seen = Instant::now();
            })
            .or_insert(ClientInfo {
                last_seen: Instant::now(),
            });
    }

    pub fn remove_inactive_clients(&mut self, timeout: Duration) {
        let now = Instant::now();
        self.clients
            .retain(|_, client| now.duration_since(client.last_seen) <= timeout);
    }

    pub async fn broadcast(
        &mut self,
        packet: &NetworkPacket<ServerMessage>,
    ) -> Result<(), std::io::Error> {
        let serialized = bincode::serialize(packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        for &addr in self.clients.keys() {
            if let Err(e) = self.socket.send_to(&serialized, addr).await {
                eprintln!("Failed to broadcast to client: {}", e);
            } else {
                self.sequence += 1;
            }
        }
        Ok(())
    }

    pub async fn send_to(
        &mut self,
        packet: &NetworkPacket<ServerMessage>,
        addr: SocketAddr,
    ) -> Result<(), std::io::Error> {
        let serialized = bincode::serialize(packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Err(e) = self.socket.send_to(&serialized, addr).await {
            eprintln!("Failed to broadcast to client: {}", e);
        } else {
            self.sequence += 1;
        }
        Ok(())
    }

    pub async fn receive(&mut self) -> Option<(ClientMessage, std::net::SocketAddr)> {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf).await {
            Ok((size, addr)) => {
                self.update_client(addr);
                match bincode::deserialize::<NetworkPacket<ClientMessage>>(&buf[..size]) {
                    Ok(packet) => {
                        if let Err(e) = self.packet_sender.send((packet.clone(), addr)).await {
                            eprintln!("Failed to send packet to handler: {}", e);
                        }
                        Some((packet.get_payload().clone(), addr))
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize packet: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive from socket: {}", e);
                None
            }
        }
    }

    pub fn get_sequence(&mut self) -> u32 {
        self.sequence += 1;
        return self.sequence;
    }

    pub fn get_timestamp(&self) -> u64 {
        // Get the current time
        let now = SystemTime::now();

        // Calculate the duration since the Unix epoch
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        // Convert the duration to milliseconds and return it as u64
        duration_since_epoch.as_millis() as u64
    }

    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            let mut buf = [0; 1024];
            let (size, addr) = self.socket.recv_from(&mut buf).await?;

            self.update_client(addr);
            match bincode::deserialize::<NetworkPacket<ClientMessage>>(&buf[..size]) {
                Ok(packet) => {
                    if let Err(e) = self.packet_sender.send((packet, addr)).await {
                        eprintln!("Failed to send packet to handler: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to deserialize packet: {}", e),
            }
        }
    }
}

impl Clone for ClientHandler {
    fn clone(&self) -> Self {
        ClientHandler {
            socket: self.socket.clone(),
            clients: self.clients.clone(),
            packet_sender: self.packet_sender.clone(),
            sequence: self.sequence,
        }
    }
}
