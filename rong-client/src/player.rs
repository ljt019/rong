use macroquad::prelude::{draw_rectangle, GREEN};

const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_HEIGHT: f32 = 10.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

pub struct Player {
    pub id: u8,
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player { id, x: 0.0, y: 0.0 }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x * SCREEN_WIDTH,
            self.y * SCREEN_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            GREEN,
        );
    }

    pub fn move_left(&mut self) {
        self.x = (self.x - 0.05).max(0.05);
    }

    pub fn move_right(&mut self) {
        self.x = (self.x + 0.05).min(0.85);
    }
}
