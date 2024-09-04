use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::net::UdpSocket;

mod game;
mod matchmaking;
mod network;

use crate::game::state::State;
use crate::matchmaking::queue::MatchmakingSystem;
use crate::network::connection::ConnectionManager;
use crate::network::packet_handler::PacketHandler;
use rong_shared::model::{self, NetworkPacket, ServerMessage};

const SOCKET_ADDR: &str = "0.0.0.0:2906";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a shared UdpSocket
    let socket = Arc::new(UdpSocket::bind(SOCKET_ADDR).await?);

    // Initialize the player manager
    let player_manager = game::player::player_manager::PlayerManager::new(Arc::clone(&socket));

    // Initialize the game state
    let mut game_state = State::new(player_manager.clone());

    // Initialize the matchmaking system
    let mut matchmaking = MatchmakingSystem::new(Duration::from_secs(60));

    // Initialize the packet handler
    let mut packet_handler = PacketHandler::new(game_state.players.clone());

    // Initialize the connection manager
    let (packet_sender, mut packet_receiver) = tokio::sync::mpsc::channel(100);
    let connection_manager = ConnectionManager::new(Arc::clone(&socket), packet_sender).await?;

    // Spawn the connection manager task
    let mut connection_manager_clone = connection_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = connection_manager_clone.run().await {
            eprintln!("Connection manager error: {}", e);
        }
    });

    // Main game loop
    loop {
        tokio::select! {
            Some((packet, addr)) = packet_receiver.recv() => {
                // Handle incoming packets
                if let Some(response) = packet_handler.handle_packet(packet, addr, &mut game_state).await {
                    // Send response packet
                    if let Err(e) = connection_manager.send_to(&response, addr).await {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(16)) => {
                // Update game state
                if let Err(e) = game_state.update().await {
                    eprintln!("Failed to update game state: {}", e);
                    continue;
                }

                // Update matchmaking
                if let Ok(new_matches) = matchmaking.update().await {
                    for _ in new_matches {
                        // Handle new matches
                        if let Err(e) = game_state.start_new_match() {
                            eprintln!("Failed to start new match: {}", e);
                        }
                    }
                }

                // Broadcast game state to all players
                let state_update = ServerMessage::GameStateChange(game_state.get_state());


                let state_packet = NetworkPacket::new(0, 0, state_update);

                if let Err(e) = connection_manager.broadcast(&state_packet).await {
                    eprintln!("Failed to broadcast state update: {}", e);
                }

                // If game started
                if model::GameState::GameStarted == game_state.get_state() && player_manager.get_players().len() < 2 {

                                let position_update = match game_state.get_positions().await {
                    Ok(positions) => ServerMessage::PositionUpdate(positions),
                    Err(e) => {
                        eprintln!("Failed to get positions: {}", e);
                        continue;
                    }
                };
                let score_update = ServerMessage::ScoreUpdate(*game_state.get_scores());

                let position_packet = NetworkPacket::new(0, 0, position_update);
                let score_packet = NetworkPacket::new(0, 0, score_update);

                if let Err(e) = connection_manager.broadcast(&position_packet).await {
                    eprintln!("Failed to broadcast position update: {}", e);
                }
                if let Err(e) = connection_manager.broadcast(&score_packet).await {
                    eprintln!("Failed to broadcast score update: {}", e);
                }
                }
            }
        }
    }
}
