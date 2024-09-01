use super::Player;
use rong_shared::{error, model};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::UdpSocket;

pub struct PlayerConnection {
    player_id: model::PlayerId,
    addr: SocketAddr,
    last_seen: Instant,
}

pub struct PlayerManager {
    players: HashMap<model::PlayerId, Player>,
    connections: HashMap<SocketAddr, PlayerConnection>,
    socket: Arc<UdpSocket>,
}

impl PlayerManager {
    pub fn new(socket: Arc<UdpSocket>) -> Self {
        Self {
            players: HashMap::new(),
            connections: HashMap::new(),
            socket,
        }
    }

    pub async fn add_player(
        &mut self,
        id: model::PlayerId,
        addr: SocketAddr,
    ) -> Result<(), error::ServerError> {
        let player = Player::new(id);
        self.players.insert(id, player);
        self.connections.insert(
            addr,
            PlayerConnection {
                player_id: id,
                addr,
                last_seen: Instant::now(),
            },
        );
        Ok(())
    }

    pub async fn remove_player(&mut self, id: model::PlayerId) -> Result<(), error::ServerError> {
        self.players.remove(&id);
        self.connections.retain(|_, conn| conn.player_id != id);
        Ok(())
    }

    pub async fn update_player_position(
        &mut self,
        id: model::PlayerId,
        dt: f32,
    ) -> Result<(), error::ServerError> {
        if let Some(player) = self.players.get_mut(&id) {
            player.update_position(dt);
        }
        Ok(())
    }

    pub async fn get_positions(&self) -> Vec<(model::PlayerId, model::Position)> {
        self.players
            .iter()
            .map(|(id, player)| (*id, player.get_position()))
            .collect()
    }

    pub async fn send_to_player(
        &self,
        id: model::PlayerId,
        msg: &[u8],
    ) -> Result<usize, error::ServerError> {
        if let Some(addr) = self
            .connections
            .values()
            .find(|conn| conn.player_id == id)
            .map(|conn| conn.addr)
        {
            Ok(self.socket.send_to(msg, addr).await?)
        } else {
            Err(error::ServerError::PlayerNotFound)
        }
    }

    pub fn get_player(&self, id: model::PlayerId) -> Option<&Player> {
        self.players.get(&id)
    }

    pub fn get_player_mut(&mut self, id: model::PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&id)
    }

    pub fn update_last_seen(&mut self, addr: SocketAddr) {
        if let Some(conn) = self.connections.get_mut(&addr) {
            conn.last_seen = Instant::now();
        }
    }

    pub fn remove_inactive_players(&mut self, timeout: Duration) {
        let now = Instant::now();
        self.connections.retain(|_, conn| {
            if now.duration_since(conn.last_seen) > timeout {
                self.players.remove(&conn.player_id);
                false
            } else {
                true
            }
        });
    }
}
