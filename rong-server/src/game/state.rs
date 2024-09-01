use super::ball::Ball;
use super::player::player_manager::PlayerManager;
use super::player::Player;
use rong_shared::error::{GameError, Result};
use rong_shared::model::{GameState, PlayerId, Position, PositionPacket, ScorePacket};
use std::time::{Duration, Instant};

pub struct State {
    pub players: PlayerManager,
    pub ball: Ball,
    state: GameState,
    scores: ScorePacket,
    last_update: Instant,
    game_duration: Duration,
}

impl State {
    pub fn new(players: PlayerManager) -> Self {
        State {
            players,
            ball: Ball::new(),
            state: GameState::WaitingForPlayers,
            scores: ScorePacket::new(0, 0),
            last_update: Instant::now(),
            game_duration: Duration::from_secs(0),
        }
    }

    pub fn start_new_match(&mut self) -> Result<()> {
        match self.state {
            GameState::WaitingForPlayers => {
                if self.players.get_player_count() == 2 {
                    self.state = GameState::GameStarted;
                    self.game_duration = Duration::from_secs(0);
                    self.ball.reset(rand::random::<u8>() % 2 + 1);
                    self.scores = ScorePacket::new(0, 0);
                    self.last_update = Instant::now();

                    // Reset player positions
                    for player in self.players.get_players_mut().values_mut() {
                        player.set_position(0.5, 0.5); // Set to center of the screen
                    }

                    Ok(())
                } else {
                    Err(GameError::Io(
                        "Not enough players to start a match".to_string(),
                    ))
                }
            }
            _ => Err(GameError::Io(
                "Cannot start a new match in the current state".to_string(),
            )),
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        match self.state {
            GameState::GameStarted => {
                self.game_duration += now.duration_since(self.last_update);
                self.update_player_positions(dt).await?;
                self.update_ball_position();
                self.handle_collisions();
                self.check_scoring();
            }
            GameState::GameOver => {
                // Do nothing
            }
            GameState::WaitingForPlayers => {
                // Check if we can start the game
                if self.players.get_player_count() == 2 {
                    self.state = GameState::GameStarted;
                    self.ball.reset(rand::random::<u8>() % 2 + 1);
                }
            }
        }
        Ok(())
    }

    async fn update_player_positions(&mut self, dt: f32) -> Result<()> {
        for player_id in [PlayerId::Player1, PlayerId::Player2].iter() {
            self.players
                .update_player_position(*player_id, dt)
                .await
                .map_err(|e| GameError::Io(e.to_string()))?;
        }
        Ok(())
    }

    fn update_ball_position(&mut self) {
        let players: Vec<Player> = self.players.get_players().values().cloned().collect();
        self.ball.update_position(&players);
    }

    fn handle_collisions(&mut self) {
        if self.ball.collides_with_wall() {
            self.ball.bounce_off_wall();
        }
    }

    fn check_scoring(&mut self) {
        if self.ball.collides_with_wall() {
            let (player1_score, player2_score) = self.scores.get_payload();
            match self.ball.which_wall() {
                "left" => {
                    self.scores = ScorePacket::new(player1_score, player2_score + 1);
                    self.ball.reset(1);
                }
                "right" => {
                    self.scores = ScorePacket::new(player1_score + 1, player2_score);
                    self.ball.reset(2);
                }
                _ => {} // Top and bottom walls don't affect score
            }
        }
    }

    pub async fn get_positions(&self) -> Result<PositionPacket> {
        let player_positions = self.players.get_positions().await;
        let ball_position = self.ball.get_position();

        let position_packet =
            PositionPacket::new(player_positions[0].1, player_positions[1].1, ball_position);

        Ok(position_packet)
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_scores(&self) -> &ScorePacket {
        &self.scores
    }

    pub fn update_score(&mut self, player_id: PlayerId) {
        match player_id {
            PlayerId::Player1 => {
                let (player1_score, player2_score) = self.scores.get_payload();
                self.scores = ScorePacket::new(player1_score + 1, player2_score);
            }
            PlayerId::Player2 => {
                let (player1_score, player2_score) = self.scores.get_payload();
                self.scores = ScorePacket::new(player1_score, player2_score + 1);
            }
        }
    }

    pub fn start_game(&mut self) -> Result<()> {
        if self.state == GameState::WaitingForPlayers {
            self.state = GameState::GameStarted;
            self.game_duration = Duration::from_secs(0);
            self.ball.reset(rand::random::<u8>() % 2 + 1);
            Ok(())
        } else {
            Err(GameError::Io("Game already started".to_string()))
        }
    }

    pub fn end_game(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn get_player_count(&self) -> usize {
        self.players.get_player_count()
    }

    pub fn move_player(&mut self, player_id: PlayerId, movement: rong_shared::model::Movement) {
        if let Some(player) = self.players.get_player_mut(player_id) {
            match movement {
                rong_shared::model::Movement::Up => player.move_up(),
                rong_shared::model::Movement::Down => player.move_down(),
                rong_shared::model::Movement::Stop => player.stop(),
            }
        }
    }

    pub fn reset(&mut self) {
        self.scores = ScorePacket::new(0, 0);
        self.ball.reset(rand::random::<u8>() % 2 + 1);
        self.state = GameState::WaitingForPlayers;
        self.game_duration = Duration::from_secs(0);
        // Reset player positions
        for player in self.players.get_players_mut().values_mut() {
            player.set_position(0.5, 0.5); // Set to center of the screen
        }
    }
}
