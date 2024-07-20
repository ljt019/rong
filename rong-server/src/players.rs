use std::io;
use std::net::{SocketAddr, UdpSocket};

pub struct Player {
    id: u8,
    pub addr: SocketAddr,
    socket: UdpSocket,
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(id: u8, addr: SocketAddr, socket: UdpSocket) -> Self {
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

    pub fn send(&self, msg: &str) -> io::Result<usize> {
        self.socket.send_to(msg.as_bytes(), self.addr)
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn move_left(&mut self) {
        self.x = (self.x - 1.0).max(0.0);
    }

    pub fn move_right(&mut self) {
        self.x = (self.x + 1.0).min(9.0);
    }
}
