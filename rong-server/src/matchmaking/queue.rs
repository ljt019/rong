use std::collections::VecDeque;
use std::time::Instant;

use crate::game::player::Player;
use rong_shared::model::GameState;

pub struct QueuedPlayer {
    player: Player,
    join_time: Instant,
}

pub struct MatchmakingQueue {
    queue: VecDeque<QueuedPlayer>,
    max_wait_time: std::time::Duration,
}

impl MatchmakingQueue {
    pub fn new(max_wait_time: std::time::Duration) -> Self {
        unimplemented!("Initialize the MatchmakingQueue")
    }

    pub fn add_player(&mut self, player: Player) {
        unimplemented!("Add a player to the queue")
    }

    pub fn remove_player(&mut self, player_id: u32) -> Option<Player> {
        unimplemented!("Remove a player from the queue")
    }

    pub fn create_matches(&mut self) -> Vec<GameState> {
        unimplemented!("Create matches from queued players")
    }

    fn find_suitable_match(&self, player: &QueuedPlayer) -> Option<&QueuedPlayer> {
        unimplemented!("Find a suitable match for the given player")
    }

    pub fn get_queue_status(&self) -> Vec<(u32, std::time::Duration)> {
        unimplemented!("Get the status of all players in the queue")
    }
}

pub struct MatchmakingSystem {
    queue: MatchmakingQueue,
}

impl MatchmakingSystem {
    pub fn new(max_wait_time: std::time::Duration) -> Self {
        unimplemented!("Initialize the MatchmakingSystem")
    }

    pub fn update(&mut self) -> Vec<GameState> {
        unimplemented!("Update the matchmaking system and create matches")
    }

    pub fn add_player(&mut self, player: Player) {
        unimplemented!("Add a player to the matchmaking system")
    }

    pub fn remove_player(&mut self, player_id: u32) -> Option<Player> {
        unimplemented!("Remove a player from the matchmaking system")
    }
}
