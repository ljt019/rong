mod ball;
mod game;
mod opponent;
mod player;
mod server;

use ball::Ball;
use game::{Game, GameState};
use opponent::Opponent;
use player::Player;
use server::Server;

use macroquad::prelude::*;

#[macroquad::main("Pong Client")]
async fn main() {
    // Set up structs for game objects
    let player = Player::new(0);
    let opponent = Opponent::new();
    let ball = Ball::new();
    let server = Server::new();

    // Set up game with the created objects
    let mut game = Game::new(server, player, opponent, ball);

    loop {
        game.update_state();

        game.draw_frame();

        match game.game_state {
            GameState::GameStarted => {
                if is_key_down(KeyCode::Left) {
                    game.move_player_left();
                }

                if is_key_down(KeyCode::Right) {
                    game.move_player_right();
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
