/*
This is the Player module. It contains the Player struct and its implementation.

The player is a struct that has the following fields:
- id: The unique id of the player
- addr: The address of the player
- socket: The UDP socket used to communicate with the player
- x: The current x position of the player
- y: The current y position of the player

The Player struct has the following methods:
- new: Creates a new Player with default values
- update_position: Updates the position of the player
- get_position: Gets the position of the player
- get_id: Gets the id of the player
- send: Sends a message to the player
- get_addr: Gets the address of the player
- move_left: Moves the player left
- move_right: Moves the player right
*/

use crate::error::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Player {
    id: u8,
    pub addr: SocketAddr,
    socket: Arc<UdpSocket>,
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(id: u8, addr: SocketAddr, socket: Arc<UdpSocket>) -> Self {
        Self {
            id,
            addr,
            socket,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32) {
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
        self.x = (self.x - 0.01).max(0.0);
    }

    pub fn move_right(&mut self) {
        self.x = (self.x + 0.01).min(9.0);
    }
}
