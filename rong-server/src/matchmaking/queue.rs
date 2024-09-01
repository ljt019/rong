use crate::game;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::Player;
use rong_shared::error::Result;
use rong_shared::model::PlayerId;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

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

    pub async fn create_matches(&mut self) -> Vec<game::state::State> {
        let mut matches = Vec::new();
        let now = Instant::now();

        while self.queue.len() >= 2 {
            let player1 = self.queue.pop_front().unwrap();
            if let Some(player2) = self.find_suitable_match(&player1) {
                let mut player_manager = PlayerManager::new(std::sync::Arc::new(
                    tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap(),
                ));
                if let Err(e) = player_manager
                    .add_player(player1.player.get_id(), player1.player.get_addr())
                    .await
                {
                    eprintln!("Failed to add player1: {:?}", e);
                    continue;
                }
                if let Err(e) = player_manager
                    .add_player(player2.get_id(), player2.get_addr())
                    .await
                {
                    eprintln!("Failed to add player2: {:?}", e);
                    continue;
                }
                let game_state = game::state::State::new(player_manager);
                matches.push(game_state);
            } else if now.duration_since(player1.join_time) > self.max_wait_time {
                // If player has waited too long, match with next available player
                if let Some(player2) = self.queue.pop_front() {
                    let mut player_manager = PlayerManager::new(std::sync::Arc::new(
                        tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap(),
                    ));
                    if let Err(e) = player_manager
                        .add_player(player1.player.get_id(), player1.player.get_addr())
                        .await
                    {
                        eprintln!("Failed to add player1: {:?}", e);
                        continue;
                    }
                    if let Err(e) = player_manager
                        .add_player(player2.player.get_id(), player2.player.get_addr())
                        .await
                    {
                        eprintln!("Failed to add player2: {:?}", e);
                        continue;
                    }
                    let game_state = game::state::State::new(player_manager);
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

    fn find_suitable_match(&mut self, _player: &QueuedPlayer) -> Option<Player> {
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

    pub async fn update(&mut self) -> Result<Vec<game::state::State>> {
        Ok(self.queue.create_matches().await)
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
