use super::shared::NetworkPacket;
use super::shared::PositionData;
use super::shared::ScoreData;
use super::GameStatus;
use super::PlayerId;
use crate::error::ServerError;

pub type ServerPacket = NetworkPacket<ServerMessage>;

pub enum ServerMessage {
    GameFound(PlayerId),
    GameUpdate(GameUpdateData),
    Success(Ack),
    Error(ServerError),
}

pub enum Ack {
    AddedToQueue,
    RemovedFromQueue,
}

pub struct GameUpdateData {
    positions: PositionData,
    scores: ScoreData,
    game_status: GameStatus,
}

impl GameUpdateData {
    pub fn new(
        position_data: PositionData,
        score_data: ScoreData,
        game_status: GameStatus,
    ) -> Self {
        GameUpdateData {
            positions: position_data,
            scores: score_data,
            game_status,
        }
    }
}
