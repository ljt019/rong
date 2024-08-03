/*

This is the Ball module. It contains the Ball struct and its implementation.

The ball is a simple struct that only has 4 fields:
- x: The current x position of the ball
- y: The current y position of the ball
- dx: The change in x position of the ball
- dy: The change in y position of the ball

The Ball struct has the following methods:
- new: Creates a new Ball with default values
- set_position: Sets the position of the ball
- get_position: Gets the position of the ball
- move_ball: Moves the ball by dx and dy
- bounce_off_player: Bounces the ball off the player
- bounce_off_wall: Bounces the ball off the wall
- collides_with_player: Checks if the ball collides with the player
- collides_with_wall: Checks if the ball collides with the wall

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
            dx: 1.0,
            dy: 1.0,
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
        self.x = self.x.max(0.0).min(9.0);
        self.y = self.y.max(0.0).min(9.0);
    }

    // Bounces the ball off the player
    pub fn bounce_off_player(&mut self, player: &Player) {
        // Get the player's position
        let (player_x, player_y) = player.get_position();

        // If the ball is at the same position as the player, reverse the direction of the ball
        // else move the ball towards the player
        if self.x < player_x {
            self.dx = -1.0;
        } else if self.x > player_x {
            self.dx = 1.0;
        }

        // If the ball is at the same position as the player, reverse the direction of the ball
        // else move the ball towards the player
        if self.y < player_y {
            self.dy = -1.0;
        } else if self.y > player_y {
            self.dy = 1.0;
        }
    }

    // Bounces the ball off the wall
    pub fn bounce_off_wall(&mut self) {
        // If the ball hits the left or right wall, reverse the x direction
        if self.x <= 0.0 || self.x >= 9.0 {
            self.dx = -self.dx;
        }

        // If the ball hits the top or bottom wall, reverse the y direction
        if self.y <= 0.0 || self.y >= 9.0 {
            self.dy = -self.dy;
        }
    }

    pub fn collides_with_player(&self, player: &Player) -> bool {
        // Get the player's position
        let (player_x, player_y) = player.get_position();

        // Check if the ball is at the same position as the player
        (self.x - player_x).abs() < 1.0 && (self.y - player_y).abs() < 1.0
    }

    pub fn collides_with_wall(&self) -> bool {
        // Check if the ball is at the edge of the board
        self.x <= 0.0 || self.x >= 9.0 || self.y <= 0.0 || self.y >= 9.0
    }
}
