mod constants;
mod game;
mod network;
mod ui;

use constants::*;
use game::{Ball, Game, GameState, Opponent, Player};
use network::Server;

use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams};
use macroquad::prelude::*;

const BALL_COLLISION_SOUND_BYTES: &[u8] = include_bytes!("../assets/wii_game_disc_case_close.wav");
const SCORE_SOUND_BYTES: &[u8] = include_bytes!("../assets/coin_collect_eleven.wav");
const MENU_MUSIC_BYTES: &[u8] = include_bytes!("../assets/menu_music.wav");

#[macroquad::main("Pong Client")]
async fn main() {
    // Setup game assets
    let ball_collision_sound = load_sound_from_bytes(BALL_COLLISION_SOUND_BYTES)
        .await
        .unwrap();

    let score_sound = load_sound_from_bytes(SCORE_SOUND_BYTES).await.unwrap();

    let menu_music = load_sound_from_bytes(MENU_MUSIC_BYTES).await.unwrap();

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

    // Flag to track if menu music is playing
    let mut menu_music_playing = false;

    loop {
        let dt = get_frame_time();

        game.update_state();
        game.player.update(dt);

        match game.game_state {
            GameState::GameStarted => {
                // Stop menu music when the game starts
                if menu_music_playing {
                    stop_sound(&menu_music);
                    menu_music_playing = false;
                }

                if is_key_down(KeyCode::Left) {
                    game.move_player_left();
                } else if is_key_down(KeyCode::Right) {
                    game.move_player_right();
                }
            }
            GameState::TitleScreen | GameState::WaitingForPlayers => {
                // Ensure menu music is playing
                if !menu_music_playing {
                    play_sound(
                        &menu_music,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.1,
                        },
                    );
                    menu_music_playing = true;
                }
            }
        }

        game.draw_frame();

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

    // Stop menu music when exiting the game
    if menu_music_playing {
        stop_sound(&menu_music);
    }
}

/*

Project File Structure:

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
