use rong_shared::{error, model};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

const PLAYER_WIDTH: f32 = 0.125; // 12.5% of screen width
const PLAYER_HEIGHT: f32 = 0.0167; // 1.67% of screen height
const MAX_SPEED: f32 = 0.2;
const ACCELERATION: f32 = 0.02;
const DECELERATION: f32 = 0.25;

#[derive(Clone)]
pub struct Player {
    id: model::PlayerId,
    position: model::Position,
    velocity: f32,
}

impl Player {
    pub fn new(id: model::PlayerId) -> Self {
        Self {
            id,
            position: (0.0, 0.0),
            velocity: 0.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let (x, _) = self.position;
        let new_x = x + self.velocity * dt;
        self.position.0 = new_x.clamp(PLAYER_WIDTH / 2.0, 1.0 - PLAYER_WIDTH / 2.0);

        // Decelerate
        if self.velocity.abs() > 0.0 {
            let deceleration = DECELERATION * dt * self.velocity.signum();
            if self.velocity.abs() > deceleration.abs() {
                self.velocity -= deceleration;
            } else {
                self.velocity = 0.0;
            }
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    pub fn get_position(&self) -> model::Position {
        self.position
    }

    pub fn get_id(&self) -> model::PlayerId {
        self.id
    }

    pub fn move_up(&mut self) {
        self.velocity = (self.velocity + ACCELERATION).min(MAX_SPEED);
    }

    pub fn move_down(&mut self) {
        self.velocity = (self.velocity - ACCELERATION).max(-MAX_SPEED);
    }

    pub fn stop(&mut self) {
        self.velocity = 0.0;
    }
}
