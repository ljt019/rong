use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

mod game;
mod matchmaking;
mod network;

use crate::game::state::State;
use crate::matchmaking::queue::MatchmakingSystem;
use crate::network::connection::ConnectionManager;
use crate::network::packet_handler::PacketHandler;
use log::info;
use rong_shared::model::{self, NetworkPacket, ServerMessage};

const SOCKET_ADDR: &str = "0.0.0.0:2906";
const TICK_RATE: Duration = Duration::from_millis(16);
const BROADCAST_INTERVAL: Duration = Duration::from_millis(50);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Create a shared UdpSocket
    let socket = Arc::new(UdpSocket::bind(SOCKET_ADDR).await?);

    // Initialize the player manager
    let player_manager = game::player::player_manager::PlayerManager::new(Arc::clone(&socket));

    // Initialize the game state
    let game_state = Arc::new(Mutex::new(State::new(player_manager.clone())));

    // Initialize the matchmaking system
    let mut matchmaking = MatchmakingSystem::new(Duration::from_secs(60));

    // Initialize the packet handler
    let packet_handler = PacketHandler::new(game_state.clone());

    // Initialize the connection manager
    let (packet_sender, mut packet_receiver) = tokio::sync::mpsc::channel(100);
    let mut connection_manager = ConnectionManager::new(Arc::clone(&socket), packet_sender).await?;

    // Spawn the connection manager task
    let mut connection_manager_clone = connection_manager.clone();
    tokio::spawn(async move {
        if let Err(e) = connection_manager_clone.run().await {
            eprintln!("Connection manager error: {}", e);
        }
    });

    let mut last_broadcast = Instant::now();

    // Main game loop
    loop {
        let loop_start = Instant::now();

        tokio::select! {
            Some((packet, addr)) = packet_receiver.recv() => {
                // Handle incoming packets
                if let Some(response) = packet_handler.handle_packet(packet.clone(), addr).await {
                    // Send response packet
                    if let Err(e) = connection_manager.send_to(&response, addr).await {
                        eprintln!("Failed to send response: {}", e);
                    }
                    info!("Received packet: {:?}", packet);
                }
            }
            _ = tokio::time::sleep(TICK_RATE) => {
                let mut state = game_state.lock().await;

                // Update game state
                if let Err(e) = state.update().await {
                    eprintln!("Failed to update game state: {}", e);
                    continue;
                }

                // Update matchmaking
                if let Ok(new_matches) = matchmaking.update().await {
                    for _ in new_matches {
                        // Handle new matches
                        if let Err(e) = state.start_new_match() {
                            eprintln!("Failed to start new match: {}", e);
                        }
                    }
                }

                // Broadcast updates at a controlled rate
                if last_broadcast.elapsed() >= BROADCAST_INTERVAL {
                    // Broadcast game state to all players
                    let state_update = ServerMessage::GameStateChange(state.get_state());
                    let state_packet = NetworkPacket::new(connection_manager.get_sequence(), connection_manager.get_timestamp(), state_update);

                    if let Err(e) = connection_manager.broadcast(&state_packet).await {
                        eprintln!("Failed to broadcast state update: {}", e);
                    } else {
                        info!("Broadcasted state update: {:?}", state_packet);
                    }

                    // If game started and we have enough players, send position and score updates
                    if state.get_state() == model::GameState::GameStarted {
                        match state.get_positions().await {
                            Ok(positions) => {
                                let position_update = ServerMessage::PositionUpdate(positions);
                                let position_packet = NetworkPacket::new(connection_manager.get_sequence(), connection_manager.get_timestamp(), position_update);
                                if let Err(e) = connection_manager.broadcast(&position_packet).await {
                                    eprintln!("Failed to broadcast position update: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to get positions: {}", e);
                            }
                        }

                        let score_update = ServerMessage::ScoreUpdate(*state.get_scores());
                        let score_packet = NetworkPacket::new(connection_manager.get_sequence(), connection_manager.get_timestamp(), score_update);
                        if let Err(e) = connection_manager.broadcast(&score_packet).await {
                            eprintln!("Failed to broadcast score update: {}", e);
                        }
                    }

                    last_broadcast = Instant::now();
                }
            }
        }

        // Ensure consistent tick rate
        let elapsed = loop_start.elapsed();
        if elapsed < TICK_RATE {
            tokio::time::sleep(TICK_RATE - elapsed).await;
        }
    }
}
