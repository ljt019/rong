use crate::game::player::player_manager::PlayerManager;
use crate::game::state::State;
use rong_shared::{
    error,
    model::{ClientMessage, Movement, NetworkPacket, PlayerId, ServerMessage},
};
use std::net::{SocketAddr, UdpSocket};

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
        game_state: &mut State,
    ) -> Option<NetworkPacket<ServerMessage>> {
        self.player_manager.update_last_seen(addr);

        match packet.get_payload() {
            ClientMessage::Connect() => {
                // Figure out which player id to assign
                let player_id_len = self.player_manager.get_players().len() as u8;

                let player_id = if player_id_len == 0 {
                    PlayerId::Player1
                } else if player_id_len == 1 {
                    PlayerId::Player2
                } else {
                    return Some(NetworkPacket::new(
                        packet.get_sequence(),
                        packet.get_timestamp(),
                        ServerMessage::Error(error::ServerError::GameFull),
                    ));
                };

                self.player_manager.add_player(player_id, addr).await.ok()?;

                println!("Player connected: {:?}", player_id);

                if player_id == PlayerId::Player2 {
                    game_state
                        .start_game()
                        .map_err(|e| {
                            println!("Error starting game: {:?}", e);
                        })
                        .expect("Failed to start game");

                    println!("Game started");
                }

                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::PlayerJoined(player_id),
                ));
            }
            ClientMessage::ConnectionAck() => {
                println!("Connection acknowledged");
                return None;
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
            _ => {
                println!("Unexpected message: {:?}", packet.get_payload());

                return Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Unexpected message".to_string()),
                ));
            }
        }
    }
}
