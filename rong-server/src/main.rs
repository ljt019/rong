mod ball;
mod game;
mod players;

use std::net::UdpSocket;

use game::{Game, GameState};

const SERVER_ADDR: &str = "0.0.0.0:2906";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(SERVER_ADDR).expect("Failed to bind to address");
    socket
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    println!("Server listening on {}", SERVER_ADDR);

    let mut game = Game::new(socket);

    loop {
        match game.get_state() {
            GameState::WaitingForPlayers => {
                if let Err(e) = game.waiting_for_players() {
                    eprintln!("Error in waiting for players: {}", e);
                }
            }
            GameState::GameStarted => {
                if let Err(e) = game.game_loop() {
                    eprintln!("Error in game loop: {}", e);
                }
            }
        }
    }
}
