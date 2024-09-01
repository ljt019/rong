use bincode;
use rong_shared::model::{ClientMessage, NetworkPacket, ServerMessage};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub async fn new(address: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Connection { socket })
    }

    pub async fn receive_packet(
        &self,
    ) -> Result<(NetworkPacket<ClientMessage>, std::net::SocketAddr), std::io::Error> {
        let mut buf = [0; 1024];
        let (size, addr) = self.socket.recv_from(&mut buf).await?;

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

        self.socket.send_to(&buf, target).await?;
        Ok(())
    }
}

pub struct ConnectionManager {
    connection: Connection,
    clients: std::collections::HashMap<std::net::SocketAddr, ClientInfo>,
    packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
}

struct ClientInfo {
    last_seen: std::time::Instant,
}

impl ConnectionManager {
    pub async fn new(
        address: &str,
        packet_sender: mpsc::Sender<(NetworkPacket<ClientMessage>, SocketAddr)>,
    ) -> Result<Self, std::io::Error> {
        let connection = Connection::new(address).await?;
        Ok(ConnectionManager {
            connection,
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
        packet: NetworkPacket<ServerMessage>,
    ) -> Result<(), std::io::Error> {
        let serialized = bincode::serialize(&packet)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        for &addr in self.clients.keys() {
            self.connection.socket.send_to(&serialized, addr).await?;
        }
        Ok(())
    }

    pub async fn handle_incoming_packets(&mut self) -> Result<(), std::io::Error> {
        loop {
            let mut buf = [0; 1024];
            match self.connection.socket.recv_from(&mut buf).await {
                Ok((size, addr)) => {
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
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No more packets to process
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub async fn run(mut self) {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.remove_inactive_clients(Duration::from_secs(300)); // 5 minutes timeout
                }
                result = self.handle_incoming_packets() => {
                    if let Err(e) = result {
                        eprintln!("Error handling incoming packets: {}", e);
                    }
                }
            }
        }
    }
}
