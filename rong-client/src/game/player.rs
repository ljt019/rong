use crate::constants::{PLAYER_HEIGHT, PLAYER_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::{draw_rectangle, Color};
use rong_shared::model;

const INTERPOLATION_FACTOR: f32 = 0.3;

pub struct Player {
    pub id: model::PlayerId,
    pub position: model::Position,
    pub target_position: model::Position,
    pub velocity: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    color: Color,
}

impl Player {
    pub fn new(id: model::PlayerId) -> Self {
        Player {
            id,
            position: (0.0, 0.0),
            target_position: (0.0, 0.0),
            velocity: 0.0,
            max_speed: 0.02,
            acceleration: 0.02,
            deceleration: 0.25,
            color: Color::new(0.5, 0.25, 0.0, 1.0),
        }
    }

    pub fn set_position(&mut self, position: model::Position) {
        self.target_position = position;
    }

    pub fn update(&mut self, dt: f32) {
        self.position.0 += (self.target_position.0 - self.position.0) * INTERPOLATION_FACTOR;
        self.position.0 += self.velocity * dt;
        self.position.0 = self.position.0.clamp(
            PLAYER_WIDTH / SCREEN_WIDTH / 2.0,
            1.0 - PLAYER_WIDTH / SCREEN_WIDTH / 2.0,
        );

        if self.velocity.abs() > 0.0 {
            let deceleration = self.deceleration * dt * self.velocity.signum();
            if self.velocity.abs() > deceleration.abs() {
                self.velocity -= deceleration;
            } else {
                self.velocity = 0.0;
            }
        }
    }

    pub fn move_left(&mut self) {
        self.velocity = (self.velocity - self.acceleration).max(-self.max_speed);
    }

    pub fn move_right(&mut self) {
        self.velocity = (self.velocity + self.acceleration).min(self.max_speed);
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.0 * SCREEN_WIDTH - PLAYER_WIDTH / 2.0,
            self.position.1 * SCREEN_HEIGHT,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            self.color,
        );
    }
}
