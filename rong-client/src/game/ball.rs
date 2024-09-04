use crate::constants::{BALL_RADIUS, SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::{draw_circle, WHITE};
use rong_shared::model;

pub struct Ball {
    position: model::Position,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: (0.5, 0.5),
        }
    }

    pub fn set_position(&mut self, position: model::Position) {
        self.position.0 = position.0;
        self.position.1 = position.1;
    }

    pub fn draw(&self) {
        draw_circle(
            self.position.0 * SCREEN_WIDTH,
            self.position.1 * SCREEN_HEIGHT,
            BALL_RADIUS,
            WHITE,
        );
    }
}
