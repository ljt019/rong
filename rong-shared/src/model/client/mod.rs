use serde::{Deserialize, Serialize};

use super::shared::Movement;
use super::shared::NetworkPacket;
use super::shared::PlayerId;

pub type ClientPacket = NetworkPacket<ClientMessage>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    JoinQueue,
    LeaveQueue,
    MovementInput(MovementData),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MovementData {
    player_id: PlayerId,
    movement: Movement,
}
