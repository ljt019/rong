use macroquad::prelude::{draw_rectangle, Color};

const PLAYER_WIDTH: f32 = 100.0;
const PLAYER_HEIGHT: f32 = 10.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

pub struct Opponent {
    pub x: f32,
    pub y: f32,
    color: Color,
}

impl Opponent {
    pub fn new() -> Self {
        Opponent {
            x: 0.0,
            y: 0.0,
            color: Color::new(0.5, 0.25, 0.0, 1.0),
        }
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
            self.color,
        );
    }
}
