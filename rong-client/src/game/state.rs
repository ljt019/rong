use super::{Ball, Opponent, Player};
use crate::network::Server;
use crate::ui::{PixelText, TitleBall, TitleText};
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use std::str::FromStr;
use std::time::Instant;

pub enum GameState {
    TitleScreen,
    WaitingForPlayers,
    GameStarted,
}

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

        let title_text = TitleText::new("RONG", screen_width() / 2.0, screen_height() / 3.0);

        let menu_color = Color::new(0.5, 0.25, 0.0, 1.0);
        let highlight_color = Color::new(1.0, 0.5, 0.0, 1.0);

        let join_game_text = PixelText::new(
            "JOIN GAME",
            screen_width() / 2.0 + -54.0,
            screen_height() / 2.0 + 50.0 + 5.0,
            1.7,
            menu_color,
            highlight_color,
        );
        let exit_text = PixelText::new(
            "EXIT",
            screen_width() / 2.0 + -29.0,
            screen_height() / 2.0 + 100.0 + -15.0,
            1.7,
            menu_color,
            highlight_color,
        );

        let title_bounds = [
            Rect::new(
                screen_width() / 2.0 - 94.0,
                screen_height() / 3.0 + 74.0,
                174.0,
                50.0,
            ),
            Rect::new(
                screen_width() / 2.0 - 54.0,
                screen_height() / 2.0 + 50.0,
                91.0,
                20.0,
            ),
            Rect::new(
                screen_width() / 2.0 - 30.0,
                screen_height() / 2.0 + 85.0,
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
            title_ball: TitleBall::new(screen_width() / 2.0, screen_height() / 2.0),
            debug_mode: true,
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
                self.handle_server_messages();
            }
        }
    }

    fn handle_server_messages(&mut self) {
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
                    self.update_game_state(received);
                }
            }
            Ok(None) => {}
            Err(e) => {
                println!("Error receiving: {}", e);
            }
        }
    }

    fn update_game_state(&mut self, received: String) {
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

    fn check_collision(&mut self, new_ball_x: f32, new_ball_y: f32) {
        let (old_x, old_y) = self.last_ball_position;
        let new_direction = (new_ball_x - old_x, new_ball_y - old_y);

        if (new_direction.0 * self.last_ball_direction.0 < 0.0)
            || (new_direction.1 * self.last_ball_direction.1 < 0.0)
        {
            println!(
                "Direction change detected! Old: {:?}, New: {:?}",
                self.last_ball_direction, new_direction
            );

            if !self.is_top_bottom_collision(old_y, new_ball_y) {
                self.play_collision_sound();
            }
        }

        self.last_ball_position = (new_ball_x, new_ball_y);
        self.last_ball_direction = new_direction;
    }

    fn is_top_bottom_collision(&self, old_y: f32, new_y: f32) -> bool {
        let top_boundary = crate::constants::BALL_RADIUS / crate::constants::SCREEN_HEIGHT;
        let bottom_boundary = 1.0 - crate::constants::BALL_RADIUS / crate::constants::SCREEN_HEIGHT;

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

                self.title_ball.update(get_frame_time(), &self.title_bounds);
                self.title_ball.draw();

                self.join_game_text
                    .draw(self.selected_option == TitleOption::JoinGame);
                self.exit_text
                    .draw(self.selected_option == TitleOption::Exit);

                // draw bounding boxes on menu text
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

                for x in (0..(crate::constants::SCREEN_WIDTH as i32)).step_by(20) {
                    draw_line(x as f32, 300.0, (x + 10) as f32, 300.0, 1.0, WHITE);
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
