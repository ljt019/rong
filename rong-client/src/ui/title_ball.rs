use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;

const BALL_RADIUS: f32 = 6.0;
const BALL_SPEED: f32 = 200.0;

pub struct TitleBall {
    position: Vec2,
    velocity: Vec2,
}

impl TitleBall {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(BALL_SPEED, BALL_SPEED).normalize() * BALL_SPEED,
        }
    }

    pub fn update(&mut self, dt: f32, bounds: &[Rect]) {
        let new_position = self.position + self.velocity * dt;

        let mut collision_occurred = false;

        for bound in bounds {
            if self.check_collision(new_position, *bound) {
                self.handle_collision(*bound);
                collision_occurred = true;
                break;
            }
        }

        if !collision_occurred {
            self.position = new_position;
        }

        // Bounce off screen edges
        if self.position.x <= BALL_RADIUS || self.position.x >= SCREEN_WIDTH - BALL_RADIUS {
            self.velocity.x = -self.velocity.x;
            self.position.x = self
                .position
                .x
                .clamp(BALL_RADIUS, SCREEN_WIDTH - BALL_RADIUS);
        }
        if self.position.y <= BALL_RADIUS || self.position.y >= SCREEN_HEIGHT - BALL_RADIUS {
            self.velocity.y = -self.velocity.y;
            self.position.y = self
                .position
                .y
                .clamp(BALL_RADIUS, SCREEN_HEIGHT - BALL_RADIUS);
        }
    }

    fn check_collision(&self, new_position: Vec2, bound: Rect) -> bool {
        let closest_x = new_position.x.clamp(bound.x, bound.x + bound.w);
        let closest_y = new_position.y.clamp(bound.y, bound.y + bound.h);

        let distance = Vec2::new(new_position.x - closest_x, new_position.y - closest_y).length();

        distance <= BALL_RADIUS
    }

    fn handle_collision(&mut self, bound: Rect) {
        let ball_center = self.position;
        let rect_center = bound.point() + bound.size() * 0.5;
        let to_ball = ball_center - rect_center;
        let abs_to_ball = to_ball.abs();

        if abs_to_ball.x > abs_to_ball.y {
            // Hit left or right side
            self.velocity.x = -self.velocity.x;
            self.position.x = if to_ball.x > 0.0 {
                bound.right() + BALL_RADIUS
            } else {
                bound.left() - BALL_RADIUS
            };
        } else {
            // Hit top or bottom
            self.velocity.y = -self.velocity.y;
            self.position.y = if to_ball.y > 0.0 {
                bound.bottom() + BALL_RADIUS
            } else {
                bound.top() - BALL_RADIUS
            };
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, BALL_RADIUS, WHITE);
    }
}
