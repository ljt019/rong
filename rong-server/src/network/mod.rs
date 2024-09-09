mod client_handler;
mod packet_handler;

use client_handler::ClientHandler;
use packet_handler::PacketHandler;

use crate::game::GameStateManager;

use std::sync::Arc;

pub struct NetworkManager {
    game_state_manager: Arc<GameStateManager>,
    client_handler: ClientHandler,
    packet_handler: PacketHandler,
}

impl NetworkManager {
    pub async fn new(game_state_manager: Arc<GameStateManager>) -> Self {
        let client_handler = ClientHandler::new().await;
        let packet_handler = PacketHandler::new(game_state_manager);

        NetworkManager {
            game_state_manager,
            client_handler,
            packet_handler,
        }
    }
}
