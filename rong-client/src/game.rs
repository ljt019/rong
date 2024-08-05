use crate::ball::Ball;
use crate::opponent::Opponent;
use crate::player::Player;
use crate::server::Server;
use std::thread;
use std::time::Duration;

use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::prelude::{clear_background, draw_text, BLACK, WHITE};
use std::str::FromStr;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 800.0;
const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_HEIGHT: f32 = 10.0;
const BALL_RADIUS: f32 = 6.0;

pub struct Game {
    pub player: Player,
    opponent: Opponent,
    ball: Ball,
    pub server: Server,
    pub game_state: GameState,
    last_received_message: String,
    score: (u8, u8),
    collision_sound: Sound,
    score_sound: Sound,
    last_ball_position: (f32, f32),
    last_ball_direction: (f32, f32),
}

impl Game {
    pub fn new(
        server: Server,
        player: Player,
        opponent: Opponent,
        ball: Ball,
        collision_sound: Sound,
        score_sound: Sound,
    ) -> Self {
        let ball_x = ball.x;
        let ball_y = ball.y;

        Game {
            server,
            player,
            opponent,
            ball,
            game_state: GameState::WaitingForPlayers,
            last_received_message: String::new(),
            score: (0, 0),
            collision_sound,
            score_sound,
            last_ball_position: (ball_x, ball_y),
            last_ball_direction: (0.0, 0.0),
        }
    }

    pub fn update_state(&mut self) {
        match self.server.receive() {
            Ok(Some(received)) => {
                if received != self.last_received_message {
                    println!("Received: {}", received);
                    self.last_received_message = received.clone();
                }

                if received == "PLAYER 1" || received == "PLAYER 2" {
                    if let Some(id) = received.split_whitespace().nth(1) {
                        if let Ok(parsed_id) = id.parse() {
                            self.player.id = parsed_id;
                            println!("Assigned as Player {}", self.player.id);
                        }
                    }
                } else if received == "GAME STARTED" {
                    self.game_state = GameState::GameStarted;
                    println!("Game started!");
                } else if received.starts_with("PLAYER ") {
                    let parts: Vec<&str> = received.split_whitespace().collect();
                    if parts.len() >= 12 {
                        let new_player_x = f32::from_str(parts[1]).unwrap_or(0.0);
                        let new_player_y = f32::from_str(parts[2]).unwrap_or(0.0);
                        let new_opponent_x = f32::from_str(parts[4]).unwrap_or(0.0);
                        let new_opponent_y = f32::from_str(parts[5]).unwrap_or(0.0);
                        let new_ball_x = f32::from_str(parts[7]).unwrap_or(0.0);
                        let new_ball_y = f32::from_str(parts[8]).unwrap_or(0.0);
                        let new_score_1 = u8::from_str(parts[10]).unwrap_or(0);
                        let new_score_2 = u8::from_str(parts[11]).unwrap_or(0);

                        self.player.set_position(new_player_x, new_player_y);
                        self.opponent.set_position(new_opponent_x, new_opponent_y);

                        self.check_collision(new_ball_x, new_ball_y);

                        self.ball.set_position(new_ball_x, new_ball_y);

                        if self.score != (new_score_1, new_score_2) {
                            self.play_score_sound();
                            self.score = (new_score_1, new_score_2);
                        }

                        self.game_state = GameState::GameStarted;
                    }
                }
            }
            Ok(None) => {
                // No data available right now, not an error
            }
            Err(e) => {
                println!("Error receiving: {}", e);
            }
        }
    }

    fn check_collision(&mut self, new_ball_x: f32, new_ball_y: f32) {
        let (old_x, old_y) = self.last_ball_position;
        let new_direction = (new_ball_x - old_x, new_ball_y - old_y);

        // Check if the ball's direction has changed significantly
        if (new_direction.0 * self.last_ball_direction.0 < 0.0)
            || (new_direction.1 * self.last_ball_direction.1 < 0.0)
        {
            println!(
                "Direction change detected! Old: {:?}, New: {:?}",
                self.last_ball_direction, new_direction
            );

            // Only play collision sound if it's not a top/bottom wall collision
            if !self.is_top_bottom_collision(old_y, new_ball_y) {
                self.play_collision_sound();
            }
        }

        // Update last known position and direction
        self.last_ball_position = (new_ball_x, new_ball_y);
        self.last_ball_direction = new_direction;
    }

    fn is_top_bottom_collision(&self, old_y: f32, new_y: f32) -> bool {
        let top_boundary = BALL_RADIUS / SCREEN_HEIGHT;
        let bottom_boundary = 1.0 - BALL_RADIUS / SCREEN_HEIGHT;

        (old_y > top_boundary && new_y <= top_boundary)
            || (old_y < bottom_boundary && new_y >= bottom_boundary)
    }

    fn play_collision_sound(&self) {
        println!("Playing collision sound!");
        play_sound(
            &self.collision_sound,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    fn play_score_sound(&self) {
        println!("Playing score sound!");
        play_sound(
            &self.score_sound,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    pub fn get_score(&self) -> (u8, u8) {
        self.score
    }

    pub fn draw_frame(&self) {
        clear_background(BLACK);

        match self.game_state {
            GameState::WaitingForPlayers => {
                draw_text(
                    "Waiting for players...",
                    10.0,
                    SCREEN_HEIGHT / 2.0,
                    20.0,
                    WHITE,
                );
            }

            GameState::GameStarted => {
                self.player.draw();
                self.opponent.draw();
                self.ball.draw();

                // Draw dotted line from left to right of the screen
                for x in (0..(SCREEN_WIDTH as i32)).step_by(20) {
                    macroquad::prelude::draw_line(
                        x as f32,
                        300.0, // Constant y-coordinate
                        (x + 10) as f32,
                        300.0, // Constant y-coordinate
                        1.0,
                        WHITE,
                    );
                }

                draw_text(
                    &format!("Score: {} - {}", self.score.0, self.score.1),
                    10.0,
                    10.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }

    pub fn move_player_left(&mut self) {
        self.player.move_left();
        self.server.send_key_press(self.player.id.to_string(), "a");
    }

    pub fn move_player_right(&mut self) {
        self.player.move_right();
        self.server.send_key_press(self.player.id.to_string(), "d");
    }
}

pub enum GameState {
    WaitingForPlayers,
    GameStarted,
}
