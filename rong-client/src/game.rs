use crate::ball::Ball;
use crate::opponent::Opponent;
use crate::player::Player;
use crate::server::Server;

use macroquad::prelude::{clear_background, draw_text, BLACK, WHITE};
use std::str::FromStr;

const SCREEN_HEIGHT: f32 = 600.0;

pub struct Game {
    player: Player,
    opponent: Opponent,
    ball: Ball,
    server: Server,
    pub game_state: GameState,
}

impl Game {
    pub fn new(server: Server, player: Player, opponent: Opponent, ball: Ball) -> Self {
        Game {
            server,
            player,
            opponent,
            ball,
            game_state: GameState::WaitingForPlayers,
        }
    }

    pub fn update_state(&mut self) {
        match self.server.receive() {
            Ok(received) => {
                println!("Received: {}", received); // Debug print

                if received.starts_with("PLAYER ") {
                    let parts: Vec<&str> = received.split_whitespace().collect();
                    if parts.len() >= 9 {
                        self.player.set_position(
                            f32::from_str(parts[1]).unwrap(),
                            f32::from_str(parts[2]).unwrap(),
                        );
                        self.opponent.set_position(
                            f32::from_str(parts[4]).unwrap(),
                            f32::from_str(parts[5]).unwrap(),
                        );
                        self.ball.set_position(
                            f32::from_str(parts[7]).unwrap(),
                            f32::from_str(parts[8]).unwrap(),
                        );
                        self.game_state = GameState::GameStarted;
                    }
                } else if received == "PLAYER 1" || received == "PLAYER 2" {
                    self.player.id = received.split_whitespace().nth(1).unwrap().parse().unwrap();
                    println!("Assigned as Player {}", self.player.id);
                } else if received == "GAME STARTED" {
                    self.game_state = GameState::GameStarted;
                }
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

                draw_text(
                    &format!("Player: ({:.2}, {:.2})", self.player.x, self.player.y),
                    10.0,
                    SCREEN_HEIGHT - 60.0,
                    20.0,
                    WHITE,
                );
                draw_text(
                    &format!("Opponent: ({:.2}, {:.2})", self.opponent.x, self.opponent.y),
                    10.0,
                    SCREEN_HEIGHT - 40.0,
                    20.0,
                    WHITE,
                );
                draw_text(
                    &format!("Ball: ({:.2}, {:.2})", self.ball.x, self.ball.y),
                    10.0,
                    SCREEN_HEIGHT - 20.0,
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
