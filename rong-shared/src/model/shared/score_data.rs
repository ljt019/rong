use super::{PlayerId, Score};

use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreData {
    player1_score: Score,
    player2_score: Score,
}

impl ScoreData {
    pub fn new(player1_score: Score, player2_score: Score) -> Self {
        Self {
            player1_score,
            player2_score,
        }
    }
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
