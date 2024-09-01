use bincode;
use rong_shared::model::{ClientMessage, NetworkPacket, ServerMessage};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};

pub struct Connection {
    socket: Arc<Mutex<UdpSocket>>,
}

impl Connection {
    pub async fn new(address: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Connection {
            socket: Arc::new(Mutex::new(socket)),
        })
    }

    pub async fn receive_packet(
        &self,
    ) -> Result<(NetworkPacket<ClientMessage>, std::net::SocketAddr), std::io::Error> {
        let mut buf = [0; 1024];
        let socket = self.socket.lock().await;
        let (size, addr) = socket.recv_from(&mut buf).await?;

        let packet: NetworkPacket<ClientMessage> = bincode::deserialize(&buf[..size])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok((packet, addr))
    }

    pub async fn send_packet(
        &self,
        packet: NetworkPacket<ServerMessage>,
        target: std::net::SocketAddr,
    ) -> Result<(), std::io::Error> {
        let buf = bincode::serialize(&packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let socket = self.socket.lock().await;
        socket.send_to(&buf, target).await?;
        Ok(())
    }
}

pub struct ConnectionManager {
    socket: Arc<UdpSocket>,
    clients: HashMap<SocketAddr, ClientInfo>,
    packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
}

#[derive(Clone)]
struct ClientInfo {
    last_seen: Instant,
}

impl ConnectionManager {
    pub async fn new(
        socket: Arc<UdpSocket>,
        packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
    ) -> Result<Self, std::io::Error> {
        Ok(ConnectionManager {
            socket,
            clients: HashMap::new(),
            packet_sender,
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
        &self,
        packet: &NetworkPacket<ServerMessage>,
    ) -> Result<(), std::io::Error> {
        let serialized = bincode::serialize(packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        for &addr in self.clients.keys() {
            self.socket.send_to(&serialized, addr).await?;
        }
        Ok(())
    }

    pub async fn send_to(
        &self,
        packet: &NetworkPacket<ServerMessage>,
        addr: SocketAddr,
    ) -> Result<(), std::io::Error> {
        let serialized = bincode::serialize(packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        self.socket.send_to(&serialized, addr).await?;

        Ok(())
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

impl Clone for ConnectionManager {
    fn clone(&self) -> Self {
        ConnectionManager {
            socket: self.socket.clone(),
            clients: self.clients.clone(),
            packet_sender: self.packet_sender.clone(),
        }
    }
}
