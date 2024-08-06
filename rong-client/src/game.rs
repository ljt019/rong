use crate::ball::Ball;
use crate::opponent::Opponent;
use crate::pixel_text::PixelText;
use crate::player::Player;
use crate::server::Server;
use crate::title_ball::TitleBall;
use crate::title_text::TitleText;
use std::thread;
use std::time::Duration;

use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::prelude::{
    clear_background, draw_rectangle_lines, draw_text, get_frame_time, is_key_pressed,
    screen_height, screen_width, Color, KeyCode, Rect, BLACK, LIME, WHITE,
};
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
    title_text: TitleText,
    join_game_text: PixelText,
    exit_text: PixelText,
    selected_option: TitleOption,
    title_ball: TitleBall,
    debug_mode: bool,
    title_bounds: [Rect; 3],
}

#[derive(PartialEq, Clone, Copy)]
enum TitleOption {
    JoinGame,
    Exit,
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

        let title_text = TitleText::new("RONG", SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 3.0);

        // Adjust color to a darker orange
        let menu_color = Color::new(0.5, 0.25, 0.0, 1.0);

        // highlight color
        let highlight_color = Color::new(1.0, 0.5, 0.0, 1.0);

        // Center the text horizontally and position it below the title
        let join_game_text = PixelText::new(
            "JOIN GAME",
            SCREEN_WIDTH / 2.0 + -54.0,
            SCREEN_HEIGHT / 2.0 + 50.0 + 5.0,
            1.7, // Reduce size
            menu_color,
            highlight_color,
        );
        let exit_text = PixelText::new(
            "EXIT",
            SCREEN_WIDTH / 2.0 + -29.0,
            SCREEN_HEIGHT / 2.0 + 100.0 + -15.0,
            1.7, // Reduce size
            menu_color,
            highlight_color,
        );

        // Initial bounds for title, join game, and exit text
        let title_bounds = [
            // Title bound
            Rect::new(
                SCREEN_WIDTH / 2.0 - 94.0,
                SCREEN_HEIGHT / 3.0 + 74.0,
                174.0,
                50.0,
            ),
            // Join Game bound
            Rect::new(
                SCREEN_WIDTH / 2.0 - 54.0,
                SCREEN_HEIGHT / 2.0 + 50.0,
                91.0,
                20.0,
            ),
            // Exit bound
            Rect::new(
                SCREEN_WIDTH / 2.0 - 30.0,
                SCREEN_HEIGHT / 2.0 + 85.0,
                40.0,
                13.0,
            ),
        ];

        Game {
            server,
            player,
            opponent,
            ball,
            game_state: GameState::TitleScreen,
            last_received_message: String::new(),
            score: (0, 0),
            collision_sound,
            score_sound,
            last_ball_position: (ball_x, ball_y),
            last_ball_direction: (0.0, 0.0),
            title_text,
            join_game_text,
            exit_text,
            selected_option: TitleOption::JoinGame,
            title_ball: TitleBall::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            debug_mode: true, // Set to false when you're done positioning
            title_bounds,
        }
    }

    pub fn update_state(&mut self) {
        match self.game_state {
            GameState::TitleScreen => {
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
                    self.selected_option = match self.selected_option {
                        TitleOption::JoinGame => TitleOption::Exit,
                        TitleOption::Exit => TitleOption::JoinGame,
                    };
                }

                if is_key_pressed(KeyCode::Enter) {
                    match self.selected_option {
                        TitleOption::JoinGame => {
                            self.server.send_connect();
                            self.game_state = GameState::WaitingForPlayers;
                        }
                        TitleOption::Exit => {
                            std::process::exit(0);
                        }
                    }
                }
            }
            GameState::WaitingForPlayers | GameState::GameStarted => {
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

    pub fn draw_frame(&mut self) {
        clear_background(BLACK);

        match self.game_state {
            GameState::TitleScreen => {
                self.title_text.update(get_frame_time());
                self.title_text.draw();

                // Update and draw the title ball
                self.title_ball.update(get_frame_time(), &self.title_bounds);
                self.title_ball.draw();

                self.join_game_text
                    .draw(self.selected_option == TitleOption::JoinGame);
                self.exit_text
                    .draw(self.selected_option == TitleOption::Exit);

                // Draw debug bounds
                //if self.debug_mode {
                //    for bound in self.title_bounds.iter() {
                //        draw_rectangle_lines(bound.x, bound.y, bound.w, bound.h, 2.0, LIME);
                //    }
                //}
            }
            GameState::WaitingForPlayers => {
                draw_text(
                    "Waiting for players...",
                    10.0,
                    screen_height() - 30.0,
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
    TitleScreen,
    WaitingForPlayers,
    GameStarted,
}
