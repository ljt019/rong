mod game_update_data;
mod network_packet;
mod position_data;
mod score_data;

pub use game_update_data::GameUpdateData;
pub use network_packet::NetworkPacket;
pub use position_data::PositionData;
pub use score_data::ScoreData;

use serde::{Deserialize, Serialize};

// Misc types
pub type Position = (f32, f32);

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Score(u8);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerId {
    Player1,
    Player2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityId {
    Player(PlayerId),
    Ball,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Movement {
    Up,
    Down,
    Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameStatus {
    WaitingForPlayers,
    GameStarted,
    GameOver,
}
