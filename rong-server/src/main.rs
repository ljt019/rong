use std::sync::Arc;
use tokio;

mod game;
mod game_server;
mod matchmaking;
mod network;

use crate::network::GameServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize the game server
    let game_server = Arc::new(GameServer::new().await?);

    // Run the game server
    game_server.run().await?;

    Ok(())
}
