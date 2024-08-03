use crate::ball::Ball;
use crate::error::Result;
use crate::players::Player;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{debug, info};

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    WaitingForPlayers,
    GameStarted,
}

pub struct Game {
    pub players: HashMap<u8, Arc<Mutex<Player>>>,
    ball: Ball,
    state: GameState,
    socket: Arc<UdpSocket>,
}

impl Game {
    pub fn new(socket: Arc<UdpSocket>) -> Self {
        Self {
            players: HashMap::new(),
            ball: Ball::new(),
            state: GameState::WaitingForPlayers,
            socket,
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub async fn connect_player(&mut self, src: SocketAddr, socket: Arc<UdpSocket>) -> Result<()> {
        info!("Player connected: {:?}", src);

        let player_count = self.players.len();

        if player_count < 2 {
            let player = Arc::new(Mutex::new(Player::new(
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
                player.update_position(0.5, 0.9);
            } else if *id == 2 {
                player.update_position(0.5, 0.1);
            }
        }

        for player in self.players.values() {
            player.lock().await.send("GAME STARTED").await?;
        }

        self.state = GameState::GameStarted;
        Ok(())
    }

    pub async fn update_game_state(&mut self) -> Result<()> {
        self.ball.move_ball();

        for player in self.players.values() {
            let player = player.lock().await;
            if self.ball.collides_with_player(&player) {
                self.ball.bounce_off_player(&player);
            }
        }

        if self.ball.collides_with_wall() {
            self.ball.bounce_off_wall();
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

            let msg = format!(
                "PLAYER {player_x} {player_y} OPPONENT {opponent_x} {opponent_y} BALL {ball_x} {ball_y}",
            );

            player.send(&msg).await?;
        }

        Ok(())
    }
}
