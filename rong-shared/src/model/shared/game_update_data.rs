use super::{GameStatus, PositionData, ScoreData};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
