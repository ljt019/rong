use bincode;
use rong_shared::model::{ClientMessage, NetworkPacket, ServerMessage};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use tracing::info;

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

        info!("*Server* Received packet from {}: {:?}", addr, packet);
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
        info!("*Server* Sent packet to {}: {:?}", target, packet);
        Ok(())
    }
}
