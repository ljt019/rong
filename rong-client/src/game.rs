use crate::ball::Ball;
use crate::opponent::Opponent;
use crate::player::Player;
use crate::server::Server;

use macroquad::prelude::{clear_background, draw_text, BLACK, WHITE};
use std::str::FromStr;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 800.0;

pub struct Game {
    player: Player,
    opponent: Opponent,
    ball: Ball,
    server: Server,
    pub game_state: GameState,
    last_received_message: String,
    score: (u8, u8),
}

impl Game {
    pub fn new(server: Server, player: Player, opponent: Opponent, ball: Ball) -> Self {
        Game {
            server,
            player,
            opponent,
            ball,
            game_state: GameState::WaitingForPlayers,
            last_received_message: String::new(),
            score: (0, 0),
        }
    }

    pub fn get_score(&self) -> (u8, u8) {
        self.score
    }

    pub fn update_state(&mut self) {
        match self.server.receive() {
            Ok(Some(received)) => {
                if received != self.last_received_message {
                    println!("Received: {}", received);
                    self.last_received_message = received.clone(); // Update the last received message
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
                } else if received.starts_with("PLAYER ") {
                    let parts: Vec<&str> = received.split_whitespace().collect();
                    if parts.len() >= 9 {
                        self.player.set_position(
                            f32::from_str(parts[1]).unwrap_or(0.0),
                            f32::from_str(parts[2]).unwrap_or(0.0),
                        );
                        self.opponent.set_position(
                            f32::from_str(parts[4]).unwrap_or(0.0),
                            f32::from_str(parts[5]).unwrap_or(0.0),
                        );
                        self.ball.set_position(
                            f32::from_str(parts[7]).unwrap_or(0.0),
                            f32::from_str(parts[8]).unwrap_or(0.0),
                        );
                        self.score = (
                            u8::from_str(parts[10]).unwrap_or(0),
                            u8::from_str(parts[11]).unwrap_or(0),
                        );
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
