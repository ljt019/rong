use crate::error::Result;
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
    id: u8,
    pub addr: SocketAddr,
    socket: Arc<UdpSocket>,
    x: f32,
    y: f32,
    velocity: f32,
}

impl Player {
    pub fn new(id: u8, addr: SocketAddr, socket: Arc<UdpSocket>) -> Self {
        Self {
            id,
            addr,
            socket,
            x: 0.0,
            y: 0.0,
            velocity: 0.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.x += self.velocity * dt;
        self.x = self.x.clamp(PLAYER_WIDTH / 2.0, 1.0 - PLAYER_WIDTH / 2.0);

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
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub async fn send(&self, msg: &str) -> Result<usize> {
        Ok(self.socket.send_to(msg.as_bytes(), self.addr).await?)
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn move_left(&mut self) {
        self.velocity = (self.velocity - ACCELERATION).max(-MAX_SPEED);
    }

    pub fn move_right(&mut self) {
        self.velocity = (self.velocity + ACCELERATION).min(MAX_SPEED);
    }
}
