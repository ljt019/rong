use crate::game::state::State;
use rong_shared::{
    error,
    model::{ClientMessage, Movement, NetworkPacket, PlayerId, ServerMessage},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PacketHandler {
    game_state: Arc<Mutex<State>>,
}

impl PacketHandler {
    pub fn new(game_state: Arc<Mutex<State>>) -> Self {
        PacketHandler { game_state }
    }

    pub async fn handle_packet(
        &self,
        packet: NetworkPacket<ClientMessage>,
        addr: SocketAddr,
    ) -> Option<NetworkPacket<ServerMessage>> {
        let mut state = self.game_state.lock().await;

        match packet.get_payload() {
            ClientMessage::Connect() => {
                // Figure out which player id to assign
                let player_id = if state.get_player_count() == 0 {
                    PlayerId::Player1
                } else if state.get_player_count() == 1 {
                    PlayerId::Player2
                } else {
                    return Some(NetworkPacket::new(
                        packet.get_sequence(),
                        packet.get_timestamp(),
                        ServerMessage::Error(error::ServerError::GameFull),
                    ));
                };

                if let Err(e) = state.add_player(player_id, addr).await {
                    eprintln!("Failed to add player: {}", e);
                    return Some(NetworkPacket::new(
                        packet.get_sequence(),
                        packet.get_timestamp(),
                        ServerMessage::Error(error::ServerError::Io(e.to_string())),
                    ));
                }

                println!("Player connected: {:?}", player_id);

                if state.get_player_count() == 2 {
                    if let Err(e) = state.start_game() {
                        eprintln!("Failed to start game: {}", e);
                    } else {
                        println!("Game started");
                    }
                }

                Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::PlayerJoined(player_id),
                ))
            }
            ClientMessage::Disconnect(player_id) => {
                state.players.remove_player(*player_id).await.ok()?;

                Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::PlayerLeft(*player_id),
                ))
            }
            ClientMessage::MovementCommand(movement_packet) => {
                let (player_id, movement) = movement_packet.get_payload();
                state.move_player(*player_id, *movement);

                Some(NetworkPacket::new(
                    packet.get_sequence(),
                    packet.get_timestamp(),
                    ServerMessage::Ack("Movement received".to_string()),
                ))
            }
            ClientMessage::ConnectionAck() => {
                println!("Connection acknowledged by client");
                None
            }
            ClientMessage::Ack(_) => None,
            ClientMessage::Error(error) => {
                println!("Error from client: {:?}", error);
                None
            }
        }
    }
}
