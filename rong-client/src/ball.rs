use macroquad::prelude::{draw_circle, WHITE};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
pub struct Ball {
    pub x: f32,
    pub y: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball { x: 0.5, y: 0.5 }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self) {
        draw_circle(self.x * SCREEN_WIDTH, self.y * SCREEN_HEIGHT, 6.0, WHITE);
    }
}
