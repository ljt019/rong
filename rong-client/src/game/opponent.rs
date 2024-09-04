use crate::constants::{PLAYER_HEIGHT, PLAYER_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::{draw_rectangle, Color};
use rong_shared::model;

pub struct Opponent {
    position: model::Position,
    color: Color,
}

impl Opponent {
    pub fn new() -> Self {
        Opponent {
            position: (0.0, 0.5),
            color: Color::new(0.5, 0.25, 0.0, 1.0),
        }
    }

    pub fn set_position(&mut self, position: model::Position) {
        self.position.0 = position.0;
        self.position.1 = position.1;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.0 * SCREEN_WIDTH,
            self.position.1 * SCREEN_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            self.color,
        );
    }
}
