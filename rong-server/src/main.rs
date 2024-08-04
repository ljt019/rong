use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::time::{interval, Duration};
use tracing::{error, info};

mod ball;
mod error;
mod game;
mod players;

use error::Result;
use game::{Game, GameState};

const SERVER_ADDR: &str = "0.0.0.0:2906";
const UPDATE_INTERVAL: Duration = Duration::from_millis(16); // ~60 FPS

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    info!("Server listening on {}", SERVER_ADDR);

    // Create and bind the UDP socket
    let socket = UdpSocket::bind(SERVER_ADDR).await?;
    let socket = Arc::new(socket);

    // Initialize the game
    let game = Arc::new(tokio::sync::Mutex::new(Game::new()));

    // Create an interval for regular updates
    let mut update_interval = interval(UPDATE_INTERVAL);

    // Main game loop
    loop {
        let game_clone = Arc::clone(&game);
        let socket_clone = Arc::clone(&socket);

        tokio::select! {
            _ = update_interval.tick() => {
                if let Err(e) = handle_game_update(game_clone.clone()).await {
                    error!("Error in game update: {}", e);
                }
            }
            _ = handle_incoming_messages(game_clone.clone(), socket_clone.clone()) => {}
        }
    }
}

// Handle regular game updates
async fn handle_game_update(game: Arc<tokio::sync::Mutex<Game>>) -> Result<()> {
    let mut game = game.lock().await;
    if game.get_state() == GameState::GameStarted {
        game.update_game_state().await?;
        game.send_game_state().await?;
    }
    Ok(())
}

// Handle incoming messages from clients
async fn handle_incoming_messages(
    game: Arc<tokio::sync::Mutex<Game>>,
    socket: Arc<UdpSocket>,
) -> Result<()> {
    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf).await?;
    let msg = std::str::from_utf8(&buf[..amt])?.to_lowercase();

    let mut game = game.lock().await;
    match game.get_state() {
        GameState::WaitingForPlayers => {
            if msg == "connect" {
                game.connect_player(src, socket).await?;
            }
        }
        GameState::GameStarted => {
            let parts: Vec<&str> = msg.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(id) = parts[0].parse::<u8>() {
                    if let Some(player) = game.players.get_mut(&id) {
                        let mut player = player.lock().await;
                        match parts[1] {
                            "a" => player.move_left(),
                            "d" => player.move_right(),
                            _ => {} // Ignore invalid movement commands
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
