use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::game;
use crate::game::player::Player;
use rong_shared::model::{GameState, PlayerId};

pub struct QueuedPlayer {
    player: Player,
    join_time: Instant,
}

pub struct MatchmakingQueue {
    queue: VecDeque<QueuedPlayer>,
    max_wait_time: Duration,
}

impl MatchmakingQueue {
    pub fn new(max_wait_time: Duration) -> Self {
        MatchmakingQueue {
            queue: VecDeque::new(),
            max_wait_time,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.queue.push_back(QueuedPlayer {
            player,
            join_time: Instant::now(),
        });
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> Option<Player> {
        if let Some(index) = self
            .queue
            .iter()
            .position(|qp| qp.player.get_id() == player_id)
        {
            Some(self.queue.remove(index).unwrap().player)
        } else {
            None
        }
    }

    pub fn create_matches(&mut self) -> Vec<GameState> {
        let mut matches = Vec::new();
        let now = Instant::now();

        while self.queue.len() >= 2 {
            let player1 = self.queue.pop_front().unwrap();
            if let Some(player2) = self.find_suitable_match(&player1) {
                let game_state = game::state::State::new(player1.player, player2.player);
                matches.push(game_state);
            } else if now.duration_since(player1.join_time) > self.max_wait_time {
                // If player has waited too long, match with next available player
                if let Some(player2) = self.queue.pop_front() {
                    let game_state = GameState::new(player1.player, player2.player);
                    matches.push(game_state);
                } else {
                    // No other players available, put back in queue
                    self.queue.push_front(player1);
                }
            } else {
                // No suitable match found and not waited too long, put back in queue
                self.queue.push_front(player1);
                break;
            }
        }

        matches
    }

    fn find_suitable_match(&mut self, player: &QueuedPlayer) -> Option<Player> {
        // For simplicity, we're just matching with the next player in queue
        // In a more advanced system, you could consider factors like skill level
        self.queue.pop_front().map(|qp| qp.player)
    }

    pub fn get_queue_status(&self) -> Vec<(PlayerId, Duration)> {
        let now = Instant::now();
        self.queue
            .iter()
            .map(|qp| (qp.player.get_id(), now.duration_since(qp.join_time)))
            .collect()
    }
}

pub struct MatchmakingSystem {
    queue: MatchmakingQueue,
}

impl MatchmakingSystem {
    pub fn new(max_wait_time: Duration) -> Self {
        MatchmakingSystem {
            queue: MatchmakingQueue::new(max_wait_time),
        }
    }

    pub fn update(&mut self) -> Vec<GameState> {
        self.queue.create_matches()
    }

    pub fn add_player(&mut self, player: Player) {
        self.queue.add_player(player);
    }

    pub fn remove_player(&mut self, player_id: PlayerId) -> Option<Player> {
        self.queue.remove_player(player_id)
    }

    pub fn get_queue_status(&self) -> Vec<(PlayerId, Duration)> {
        self.queue.get_queue_status()
    }
}
