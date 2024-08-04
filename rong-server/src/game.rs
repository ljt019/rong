use crate::ball::Ball;
use crate::error::Result;
use crate::players::Player;

use futures::future;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{error, info};

const UPDATE_RATE: f32 = 60.0; // Updates per second
const DT: f32 = 1.0 / UPDATE_RATE;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
}

pub struct Game {
    pub players: HashMap<u8, Arc<Mutex<Player>>>,
    ball: Ball,
    state: GameState,
    score: (u8, u8),
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            ball: Ball::new(),
            state: GameState::WaitingForPlayers,
            score: (0, 0),
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn get_score(&self) -> (u8, u8) {
        self.score
    }

    pub fn add_point(&mut self, player_id: u8) {
        if player_id == 1 {
            self.score.0 += 1;
        } else if player_id == 2 {
            self.score.1 += 1;
        }
    }

    pub async fn connect_player(&mut self, src: SocketAddr, socket: Arc<UdpSocket>) -> Result<()> {
        info!("Player connected: {:?}", src);

        let player_count = self.players.len();

        if player_count < 2 {
            let player: Arc<Mutex<Player>> = Arc::new(Mutex::new(Player::new(
                player_count as u8 + 1,
                src,
                Arc::clone(&socket),
            )));

            self.players
                .insert(player.lock().await.get_id(), Arc::clone(&player));

            let response = if player_count == 0 {
                "PLAYER 1"
            } else {
                "PLAYER 2"
            };

            player.lock().await.send(response).await?;

            if self.players.len() == 2 {
                self.start_game().await?;
            }
        }

        Ok(())
    }

    async fn start_game(&mut self) -> Result<()> {
        info!("Game starting");

        self.ball.set_position(0.5, 0.5);

        for (id, player) in self.players.iter_mut() {
            let mut player = player.lock().await;
            if *id == 1 {
                player.set_position(0.5, 0.9); // Player 1 at the bottom
            } else if *id == 2 {
                player.set_position(0.5, 0.1); // Player 2 at the top
            }
        }

        for player in self.players.values() {
            player.lock().await.send("GAME STARTED").await?;
        }

        self.state = GameState::GameStarted;
        Ok(())
    }

    pub async fn update_game_state(&mut self) -> Result<()> {
        let players: Vec<Player> = future::join_all(
            self.players
                .values()
                .map(|p| async { p.lock().await.clone() }),
        )
        .await;

        for player in &players {
            let mut player = self.players.get_mut(&player.get_id()).unwrap().lock().await;
            player.update_position(DT);
        }

        self.ball.update_position(&players);

        if self.ball.collides_with_wall() {
            let wall: String = self.ball.which_wall().to_string();
            match wall.as_str() {
                "top" => {
                    info!("Top wall declared: {wall}");
                    self.add_point(2);
                    self.ball.set_position(0.5, 0.5);
                    self.ball.reset_velocity(2);
                }
                "bottom" => {
                    info!("Bottom wall declared: {wall}");
                    self.add_point(1);
                    self.ball.set_position(0.5, 0.5);
                    self.ball.reset_velocity(1);
                }
                "right" | "left" => {
                    info!(
                        "{} wall declared: {wall}",
                        if wall == "right" { "Right" } else { "Left" }
                    );
                    self.ball.bounce_off_wall();
                }
                _ => {
                    let (ball_x, ball_y) = self.ball.get_position();
                    error!("Unknown wall, coordinates: {wall} {ball_x} {ball_y}");
                }
            }
        }

        Ok(())
    }

    pub async fn send_game_state(&self) -> Result<()> {
        for (id, player) in &self.players {
            let opponent_id = if *id == 1 { 2 } else { 1 };

            let player = player.lock().await;
            let (player_x, player_y) = player.get_position();

            let opponent = self.players.get(&opponent_id).unwrap().lock().await;
            let (opponent_x, opponent_y) = opponent.get_position();

            let (ball_x, ball_y) = self.ball.get_position();

            let score_p1 = self.score.0;
            let score_p2 = self.score.1;

            let msg = format!(
                "PLAYER {player_x} {player_y} OPPONENT {opponent_x} {opponent_y} BALL {ball_x} {ball_y} SCORE {score_p1} {score_p2}",
            );

            player.send(&msg).await?;
        }

        Ok(())
    }
}
