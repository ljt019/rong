mod constants;
mod game;
mod network;
mod ui;

use constants::*;
use game::{Ball, Game, Opponent, Player};
use network::Server;

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};
use macroquad::prelude::*;

const BALL_COLLISION_SOUND_BYTES: &[u8] = include_bytes!("../assets/wii_game_disc_case_close.wav");
const SCORE_SOUND_BYTES: &[u8] = include_bytes!("../assets/coin_collect_eleven.wav");

#[macroquad::main("Pong Client")]
async fn main() {
    // Setup game assets
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

    loop {
        let dt = get_frame_time();

        game.update_state();
        game.player.update(dt);

        match game.game_state {
            game::GameState::GameStarted => {
                if is_key_down(KeyCode::Left) {
                    game.move_player_left();
                } else if is_key_down(KeyCode::Right) {
                    game.move_player_right();
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

/*

src/
├── main.rs
├── game/
│   ├── mod.rs
│   ├── ball.rs
│   ├── player.rs
│   ├── opponent.rs
│   └── state.rs
├── ui/
│   ├── mod.rs
│   ├── pixel_text.rs
│   ├── title_text.rs
│   └── title_ball.rs
├── network/
│   ├── mod.rs
│   └── server.rs
└── constants.rs

*/
