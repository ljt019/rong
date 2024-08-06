mod ball;
mod game;
mod opponent;
mod pixel_text;
mod player;
mod server;
mod title_ball;
mod title_text;

use ball::Ball;
use game::{Game, GameState};
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};
use macroquad::prelude::*;
use opponent::Opponent;
use player::Player;
use server::Server;
use std::time::Instant;

const MOVE_COOLDOWN_SECONDS: f32 = 0.1; // 100ms

const BALL_COLLISION_SOUND_BYTES: &[u8] = include_bytes!("../assets/wii_game_disc_case_close.wav");

const SCORE_SOUND_BYTES: &[u8] = include_bytes!("../assets/coin_collect_eleven.wav");

#[macroquad::main("Pong Client")]
async fn main() {
    //setup game assets
    let ball_collision_sound = load_sound_from_bytes(BALL_COLLISION_SOUND_BYTES)
        .await
        .unwrap();

    let score_sound = load_sound_from_bytes(SCORE_SOUND_BYTES).await.unwrap();

    // Set up structs for game objects
    let player = Player::new(0);
    let opponent = Opponent::new();
    let ball = Ball::new();
    let server = Server::new();
    // Set up game with the created objects
    let mut game = Game::new(
        server,
        player,
        opponent,
        ball,
        ball_collision_sound,
        score_sound,
    );

    let mut last_update = Instant::now();

    loop {
        let dt = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        game.update_state();
        game.player.update(dt);

        match game.game_state {
            GameState::GameStarted => {
                if is_key_down(KeyCode::Left) {
                    game.player.move_left();
                    game.server.send_key_press(game.player.id.to_string(), "a");
                } else if is_key_down(KeyCode::Right) {
                    game.player.move_right();
                    game.server.send_key_press(game.player.id.to_string(), "d");
                }
            }
            _ => {}
        }

        game.draw_frame();

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
