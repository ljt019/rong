mod network_packet;
mod position_data;
mod score_data;

pub use network_packet::NetworkPacket;
pub use position_data::PositionData;
pub use score_data::ScoreData;

// Misc types
pub type Position = (f32, f32);

pub struct Score(u8);

pub enum PlayerId {
    Player1,
    Player2,
}

pub enum EntityId {
    Player(PlayerId),
    Ball,
}

pub enum Movement {
    Up,
    Down,
    Stop,
}

pub enum GameStatus {
    WaitingForPlayers,
    GameStarted,
    GameOver,
}
