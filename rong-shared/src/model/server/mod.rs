use super::shared::{GameUpdateData, NetworkPacket, PlayerId};
use crate::error::ServerError;

use serde::{Deserialize, Serialize};

pub type ServerPacket = NetworkPacket<ServerMessage>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    GameFound(PlayerId),
    GameUpdate(GameUpdateData),
    Success(Ack),
    Error(ServerError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ack {
    AddedToQueue,
    RemovedFromQueue,
}
