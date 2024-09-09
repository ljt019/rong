use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::game::GameStateManager;
use crate::matchmaking::MatchmakingManager;
use crate::network::NetworkManager;

const TICK_RATE: Duration = Duration::from_millis(16);
const BROADCAST_INTERVAL: Duration = Duration::from_millis(50);

pub struct GameServer {
    network_manager: Arc<Mutex<NetworkManager>>,
    matchmaking_manager: Arc<Mutex<MatchmakingManager>>,
    game_state_manager: Arc<Mutex<GameStateManager>>,
}

impl GameServer {}
