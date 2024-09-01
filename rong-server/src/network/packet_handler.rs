use crate::game::player::player_manager::PlayerManager;
use crate::game::state::State;
use rong_shared::model::{ClientMessage, Movement, NetworkPacket, ServerMessage};
use std::net::SocketAddr;

pub struct PacketHandler {
    player_manager: PlayerManager,
}

impl PacketHandler {
    pub fn new(player_manager: PlayerManager) -> Self {
        PacketHandler { player_manager }
    }

    pub async fn handle_packet(
        &mut self,
        packet: NetworkPacket<ClientMessage>,
        addr: SocketAddr,
        _game_state: &mut State,
    ) -> Option<NetworkPacket<ServerMessage>> {
        self.player_manager.update_last_seen(addr);

        match packet.get_payload() {
            ClientMessage::Connect(player_id) => {
                self.player_manager
                    .add_player(*player_id, addr)
                    .await
                    .ok()?;

                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Connection successful".to_string()),
                ));
            }
            ClientMessage::Disconnect(player_id) => {
                self.player_manager.remove_player(*player_id).await.ok()?;

                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Disconnection successful".to_string()),
                ));
            }
            ClientMessage::MovementCommand(movement_packet) => {
                // Update player position with handler
                let (player_id, movement) = movement_packet.get_payload();

                match movement {
                    Movement::Up => self
                        .player_manager
                        .update_player_position(*player_id, 1.0)
                        .await
                        .ok()?,
                    Movement::Down => self
                        .player_manager
                        .update_player_position(*player_id, -1.0)
                        .await
                        .ok()?,
                    Movement::Stop => self
                        .player_manager
                        .update_player_position(*player_id, 0.0)
                        .await
                        .ok()?,
                }

                // Ack movement received
                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Movement received".to_string()),
                ));
            }
            ClientMessage::Ack(_) => None,
            ClientMessage::Error(error) => {
                println!("Error: {}", error);

                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Error received".to_string()),
                ));
            }
        }
    }
}
