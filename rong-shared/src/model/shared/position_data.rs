use crate::model::EntityId;
use crate::model::PlayerId;
use crate::model::Position;

use std::ops::{Index, IndexMut};
pub struct PositionData {
    player1_position: Position,
    player2_position: Position,
    ball_position: Position,
}

impl PositionData {
    pub fn new(
        player1_position: Position,
        player2_position: Position,
        ball_position: Position,
    ) -> Self {
        PositionData {
            player1_position,
            player2_position,
            ball_position,
        }
    }
}

impl Index<EntityId> for PositionData {
    type Output = Position;

    fn index(&self, entity_id: EntityId) -> &Self::Output {
        match entity_id {
            EntityId::Player(PlayerId::Player1) => &self.player1_position,
            EntityId::Player(PlayerId::Player2) => &self.player2_position,
            EntityId::Ball => &self.ball_position,
        }
    }
}

impl IndexMut<EntityId> for PositionData {
    fn index_mut(&mut self, entity_id: EntityId) -> &mut Self::Output {
        match entity_id {
            EntityId::Player(PlayerId::Player1) => &mut self.player1_position,
            EntityId::Player(PlayerId::Player2) => &mut self.player2_position,
            EntityId::Ball => &mut self.ball_position,
        }
    }
}
