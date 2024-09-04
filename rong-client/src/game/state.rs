use super::{Ball, Opponent, Player};
use crate::constants::{BALL_RADIUS, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::network::Server;
use crate::ui::{PixelText, TitleBall, TitleText};
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use rong_shared::error::ClientError;
use rong_shared::model::{GameState, Movement, PlayerId, Position, ServerMessage};

#[derive(PartialEq, Clone, Copy)]
pub enum ClientState {
    TitleScreen,
    WaitingForPlayers,
    Playing,
    GameOver,
}

#[derive(PartialEq, Clone, Copy)]
enum TitleOption {
    JoinGame,
    Exit,
}

pub struct Game {
    pub player: Player,
    opponent: Opponent,
    ball: Ball,
    pub server: Server,
    pub client_state: ClientState,
    server_game_state: GameState,
    score: (u8, u8),
    collision_sound: Sound,
    score_sound: Sound,
    last_ball_position: Position,
    last_ball_direction: (f32, f32),
    title_text: TitleText,
    join_game_text: PixelText,
    exit_text: PixelText,
    selected_option: TitleOption,
    title_ball: TitleBall,
    debug_mode: bool,
    title_bounds: [Rect; 3],
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
        let title_text = TitleText::new("RONG", SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 3.0);

        let menu_color = Color::new(0.5, 0.25, 0.0, 1.0);
        let highlight_color = Color::new(1.0, 0.5, 0.0, 1.0);

        let join_game_text = PixelText::new(
            "JOIN GAME",
            SCREEN_WIDTH / 2.0 - 54.0,
            SCREEN_HEIGHT / 2.0 + 55.0,
            1.7,
            menu_color,
            highlight_color,
        );
        let exit_text = PixelText::new(
            "EXIT",
            SCREEN_WIDTH / 2.0 - 29.0,
            SCREEN_HEIGHT / 2.0 + 85.0,
            1.7,
            menu_color,
            highlight_color,
        );

        let title_bounds = [
            Rect::new(
                SCREEN_WIDTH / 2.0 - 94.0,
                SCREEN_HEIGHT / 3.0 + 74.0,
                174.0,
                50.0,
            ),
            Rect::new(
                SCREEN_WIDTH / 2.0 - 54.0,
                SCREEN_HEIGHT / 2.0 + 50.0,
                91.0,
                20.0,
            ),
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
            client_state: ClientState::TitleScreen,
            server_game_state: GameState::WaitingForPlayers,
            score: (0, 0),
            collision_sound,
            score_sound,
            last_ball_position: (0.5, 0.5),
            last_ball_direction: (0.0, 0.0),
            title_text,
            join_game_text,
            exit_text,
            selected_option: TitleOption::JoinGame,
            title_ball: TitleBall::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            debug_mode: false,
            title_bounds,
        }
    }

    pub fn update_state(&mut self) -> Result<(), ClientError> {
        match self.client_state {
            ClientState::TitleScreen => {
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
                    self.selected_option = match self.selected_option {
                        TitleOption::JoinGame => TitleOption::Exit,
                        TitleOption::Exit => TitleOption::JoinGame,
                    };
                }

                if is_key_pressed(KeyCode::Enter) {
                    match self.selected_option {
                        TitleOption::JoinGame => {
                            self.server.send_connect()?;
                            self.client_state = ClientState::WaitingForPlayers;
                        }
                        TitleOption::Exit => {
                            std::process::exit(0);
                        }
                    }
                }
            }
            ClientState::WaitingForPlayers => {
                // No action needed, just wait for server to start the game
            }
            ClientState::Playing => {
                if is_key_down(KeyCode::Left) {
                    self.server.send_movement(Movement::Down)?;
                } else if is_key_down(KeyCode::Right) {
                    self.server.send_movement(Movement::Up)?;
                } else {
                    self.server.send_movement(Movement::Stop)?;
                }
            }
            ClientState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset_game()?;
                }
            }
        }

        self.handle_server_messages()?;
        Ok(())
    }

    fn handle_server_messages(&mut self) -> Result<(), ClientError> {
        while let Some(message) = self.server.receive()? {
            match message {
                ServerMessage::GameStateChange(new_state) => {
                    self.server_game_state = new_state;
                    match new_state {
                        GameState::GameStarted => self.client_state = ClientState::Playing,
                        GameState::GameOver => self.client_state = ClientState::GameOver,
                        _ => {}
                    }
                }
                ServerMessage::PositionUpdate(positions) => {
                    let (player1, player2, ball) = positions.get_payload();
                    if self.player.id == PlayerId::Player1 {
                        self.player.set_position((player1.0, player1.1));
                        self.opponent.set_position((player2.0, player2.1));
                    } else {
                        self.player.set_position((player2.0, player2.1));
                        self.opponent.set_position((player2.0, player2.1));
                    }
                    self.check_collision(*ball);
                    self.ball.set_position((ball.0, ball.1));
                }
                ServerMessage::ScoreUpdate(scores) => {
                    let (score1, score2) = scores.get_payload();
                    if self.score != (score1, score2) {
                        self.play_score_sound();
                        self.score = (score1, score2);
                    }
                }
                ServerMessage::PlayerJoined(_) | ServerMessage::PlayerLeft(_) => {
                    // Update UI to show player connection status if needed
                }
                ServerMessage::Ack(_) => {
                    // Handle acknowledgement if needed
                }
                ServerMessage::Error(error) => {
                    println!("Server error: {:?}", error);
                    // Handle server error, possibly disconnecting or showing an error message
                }
            }
        }
        Ok(())
    }

    pub fn move_player_left(&mut self) -> Result<(), ClientError> {
        self.server.send_movement(Movement::Down)?;
        Ok(())
    }

    pub fn move_player_right(&mut self) -> Result<(), ClientError> {
        self.server.send_movement(Movement::Up)?;
        Ok(())
    }

    fn check_collision(&mut self, new_ball_pos: Position) {
        let (old_x, old_y) = self.last_ball_position;
        let new_direction = (new_ball_pos.0 - old_x, new_ball_pos.1 - old_y);

        if (new_direction.0 * self.last_ball_direction.0 < 0.0)
            || (new_direction.1 * self.last_ball_direction.1 < 0.0)
        {
            if !self.is_top_bottom_collision(old_y, new_ball_pos.1) {
                self.play_collision_sound();
            }
        }

        self.last_ball_position = new_ball_pos;
        self.last_ball_direction = new_direction;
    }

    fn is_top_bottom_collision(&self, old_y: f32, new_y: f32) -> bool {
        let top_boundary = BALL_RADIUS / SCREEN_HEIGHT;
        let bottom_boundary = 1.0 - BALL_RADIUS / SCREEN_HEIGHT;

        (old_y > top_boundary && new_y <= top_boundary)
            || (old_y < bottom_boundary && new_y >= bottom_boundary)
    }

    fn play_collision_sound(&self) {
        play_sound(
            &self.collision_sound,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    fn play_score_sound(&self) {
        play_sound(
            &self.score_sound,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }

    pub fn draw_frame(&mut self) {
        clear_background(BLACK);

        match self.client_state {
            ClientState::TitleScreen => {
                self.title_text.update(get_frame_time());
                self.title_text.draw();

                self.title_ball.update(get_frame_time(), &self.title_bounds);
                self.title_ball.draw();

                self.join_game_text
                    .draw(self.selected_option == TitleOption::JoinGame);
                self.exit_text
                    .draw(self.selected_option == TitleOption::Exit);

                if self.debug_mode {
                    for bound in self.title_bounds.iter() {
                        draw_rectangle_lines(bound.x, bound.y, bound.w, bound.h, 2.0, LIME);
                    }
                }
            }
            ClientState::WaitingForPlayers => {
                draw_text(
                    "Waiting for players...",
                    10.0,
                    SCREEN_HEIGHT - 30.0,
                    20.0,
                    WHITE,
                );
            }
            ClientState::Playing | ClientState::GameOver => {
                self.player.draw();
                self.opponent.draw();
                self.ball.draw();

                for x in (0..(SCREEN_WIDTH as i32)).step_by(20) {
                    draw_line(x as f32, 300.0, (x + 10) as f32, 300.0, 1.0, WHITE);
                }

                draw_text(
                    &format!("Score: {} - {}", self.score.0, self.score.1),
                    10.0,
                    30.0,
                    20.0,
                    WHITE,
                );

                if self.client_state == ClientState::GameOver {
                    let game_over_text = "Game Over! Press Enter to play again";
                    let text_dimensions = measure_text(game_over_text, None, 30, 1.0);
                    draw_text(
                        game_over_text,
                        (SCREEN_WIDTH - text_dimensions.width) / 2.0,
                        SCREEN_HEIGHT / 2.0,
                        30.0,
                        WHITE,
                    );
                }
            }
        }

        if self.debug_mode {
            draw_text(&format!("FPS: {}", get_fps()), 10.0, 10.0, 20.0, GREEN);
        }
    }

    fn reset_game(&mut self) -> Result<(), ClientError> {
        self.score = (0, 0);
        self.client_state = ClientState::WaitingForPlayers;
        self.server_game_state = GameState::WaitingForPlayers;
        self.server.send_connect()?;
        Ok(())
    }

    pub fn toggle_debug_mode(&mut self) {
        self.debug_mode = !self.debug_mode;
    }
}
