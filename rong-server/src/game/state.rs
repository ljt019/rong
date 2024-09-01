use super::ball::Ball;
use super::player::player_manager::PlayerManager;
use rong_shared::model;

pub struct State {
    pub players: PlayerManager,
    pub ball: Ball,
    state: model::GameState,
}

impl State {
    pub fn new(players: PlayerManager, ball: Ball) -> Self {
        State {
            players,
            ball,
            state: model::GameState::WaitingForPlayers,
        }
    }

    pub async fn get_positions(&self) -> (model::Position, model::Position, model::Position) {
        let player_positions = self.players.get_positions().await;
        let ball_position = self.ball.get_position();
        (player_positions[0].1, player_positions[1].1, ball_position)
    }

    /*

    enum GameState {
        WaitingForPlayers,
        Playing,
        GameOver,
    }

    */

    pub async fn get_state(&self) -> model::GameState {
        self.state
    }
}
