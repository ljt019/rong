mod ball;
mod game;
mod opponent;
mod player;
mod server;

use ball::Ball;
use game::{Game, GameState};
use macroquad::prelude::*;
use opponent::Opponent;
use player::Player;
use server::Server;

const MOVE_COOLDOWN: f32 = 0.1; // Cooldown time in seconds

#[macroquad::main("Pong Client")]
async fn main() {
    // Set up structs for game objects
    let player = Player::new(0);
    let opponent = Opponent::new();
    let ball = Ball::new();
    let server = Server::new();
    // Set up game with the created objects
    let mut game = Game::new(server, player, opponent, ball);

    let mut last_move_time = 0.0;

    loop {
        game.update_state();
        game.draw_frame();

        let current_time = get_time() as f32;

        match game.game_state {
            GameState::GameStarted => {
                if current_time - last_move_time >= MOVE_COOLDOWN {
                    if is_key_down(KeyCode::Left) {
                        game.move_player_left();
                        last_move_time = current_time;
                    } else if is_key_down(KeyCode::Right) {
                        game.move_player_right();
                        last_move_time = current_time;
                    }
                }
            }
            _ => {}
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
