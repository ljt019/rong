/*
This is the Ball module. It contains the Ball struct and its implementation.

The ball is a struct that has the following fields:
- x: The current x position of the ball
- y: The current y position of the ball
- dx: The change in x position of the ball
- dy: The change in y position of the ball

*/

use crate::players::Player;

pub struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            dx: 0.01,
            dy: 0.01,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn move_ball(&mut self) {
        // Move the ball by dx and dy
        self.x += self.dx;
        self.y += self.dy;

        // Keep the ball within bounds
        self.x = self.x.clamp(0.0, 1.0);
        self.y = self.y.clamp(0.0, 1.0);
    }

    // Bounces the ball off the player
    pub fn bounce_off_player(&mut self, player: &Player) {
        // Get the player's position
        let (player_x, player_y) = player.get_position();

        // If the ball is at the same position as the player, reverse the direction of the ball
        // else move the ball towards the player
        self.dx = if self.x < player_x {
            -0.01
        } else if self.x > player_x {
            0.01
        } else {
            self.dx
        };

        self.dy = if self.y < player_y {
            -0.01
        } else if self.y > player_y {
            0.01
        } else {
            self.dy
        };
    }

    // Bounces the ball off the wall
    pub fn bounce_off_wall(&mut self) {
        // If the ball hits the left or right wall, reverse the x direction
        if self.x <= 0.0 || self.x >= 1.0 {
            self.dx = -self.dx;
        }

        // If the ball hits the top or bottom wall, reverse the y direction
        if self.y <= 0.0 || self.y >= 1.0 {
            self.dy = -self.dy;
        }
    }

    pub fn collides_with_player(&self, player: &Player) -> bool {
        // Get the player's position
        let (player_x, player_y) = player.get_position();

        // Check if the ball is close to the player
        // We use a slightly larger collision box to account for the smaller movement increments
        (self.x - player_x).abs() < 0.07 && (self.y - player_y).abs() < 0.02
    }

    pub fn collides_with_wall(&self) -> bool {
        // Check if the ball is at the edge of the screen
        self.x <= 0.0 || self.x >= 1.0 || self.y <= 0.0 || self.y >= 1.0
    }

    pub fn which_wall(&mut self) -> &str {
        // Define 4 walls
        let top_wall = (0.0..1.0, 0.0..0.02);
        let bottom_wall = (0.0..1.0, 0.98..1.0);
        let left_wall = (0.0..0.02, 0.0..1.0);
        let right_wall = (0.98..1.0, 0.0..1.0);

        // Check which wall the ball is colliding with
        match (self.x, self.y) {
            (x, y) if top_wall.0.contains(&x) && top_wall.1.contains(&y) => "top",
            (x, y) if bottom_wall.0.contains(&x) && bottom_wall.1.contains(&y) => "bottom",
            (x, y) if left_wall.0.contains(&x) && left_wall.1.contains(&y) => "left",
            (x, y) if right_wall.0.contains(&x) && right_wall.1.contains(&y) => "right",
            _ => "none",
        }
    }
}
