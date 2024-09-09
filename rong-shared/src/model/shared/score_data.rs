use crate::model::PlayerId;
use crate::model::Score;

use std::ops::{Index, IndexMut};
pub struct ScoreData {
    player1_score: Score,
    player2_score: Score,
}

impl Index<PlayerId> for ScoreData {
    type Output = Score;

    fn index(&self, player_id: PlayerId) -> &Self::Output {
        match player_id {
            PlayerId::Player1 => &self.player1_score,
            PlayerId::Player2 => &self.player2_score,
        }
    }
}

impl IndexMut<PlayerId> for ScoreData {
    fn index_mut(&mut self, index: PlayerId) -> &mut Self::Output {
        match index {
            PlayerId::Player1 => &mut self.player1_score,
            PlayerId::Player2 => &mut self.player2_score,
        }
    }
}
