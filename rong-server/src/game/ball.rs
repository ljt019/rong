use super::player::Player;
use rand::Rng;
use rong_shared::model;

// Constants for game dimensions and ball behavior
const PLAYER_WIDTH: f32 = 0.125; // 12.5% of screen width
const PLAYER_HEIGHT: f32 = 0.0167; // 1.67% of screen height
const MAX_BALL_SPEED: f32 = 0.02; // Maximum ball speed to prevent tunneling
const INITIAL_BALL_SPEED: f32 = 0.005; // Initial ball speed after reset

pub struct Ball {
    x: f32,
    y: f32,
    dx: f32, // Velocity in x direction
    dy: f32, // Velocity in y direction
    radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        let mut ball = Self {
            x: 0.5,
            y: 0.5,
            dx: 0.0,
            dy: 0.0,
            radius: 0.01, // 1% of screen width/height
        };
        ball.reset(rand::random::<u8>() % 2 + 1); // Randomly serve to player 1 or 2
        ball
    }

    // Reset the ball's position and set its initial trajectory
    pub fn reset(&mut self, serve_to_player: u8) {
        let mut rng = rand::thread_rng();

        // Reset position to center
        self.x = 0.5;
        self.y = 0.5;

        // Randomize horizontal direction slightly
        let angle = rng.gen_range(-std::f32::consts::PI / 6.0..std::f32::consts::PI / 6.0);

        // Set initial speed and direction
        self.dx = INITIAL_BALL_SPEED * angle.sin();
        self.dy = if serve_to_player == 1 {
            INITIAL_BALL_SPEED * angle.cos()
        } else {
            -INITIAL_BALL_SPEED * angle.cos()
        };
    }

    // Set the ball's position, ensuring it stays within bounds
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x.clamp(self.radius, 1.0 - self.radius);
        self.y = y.clamp(self.radius, 1.0 - self.radius);
    }

    // Get the current position of the ball
    pub fn get_position(&self) -> model::Position {
        (self.x, self.y)
    }

    // Update the ball's position, checking for collisions
    pub fn update_position(&mut self, players: &[Player]) {
        // Calculate the number of steps to move the ball
        // This helps prevent tunneling by ensuring small movements
        let steps = (self.dx.abs().max(self.dy.abs()) / 0.01).ceil() as i32;
        let step_x = self.dx / steps as f32;
        let step_y = self.dy / steps as f32;

        // Move the ball step by step
        for _ in 0..steps {
            let new_x = self.x + step_x;
            let new_y = self.y + step_y;

            let mut collision_occurred = false;

            // Check for collisions with each player
            for player in players {
                if self.check_collision(new_x, new_y, player) {
                    self.handle_collision(player);
                    collision_occurred = true;
                    break;
                }
            }

            // If no collision occurred, update the ball's position
            if !collision_occurred {
                self.x = new_x.clamp(self.radius, 1.0 - self.radius);
                self.y = new_y.clamp(self.radius, 1.0 - self.radius);
            }
        }
    }

    // Check if the ball collides with a player's paddle
    fn check_collision(&self, new_x: f32, new_y: f32, player: &Player) -> bool {
        let (player_x, player_y) = player.get_position();

        // Calculate the boundaries of the player's paddle
        let player_left = player_x - PLAYER_WIDTH / 2.0;
        let player_right = player_x + PLAYER_WIDTH / 2.0;
        let player_top = player_y - PLAYER_HEIGHT / 2.0;
        let player_bottom = player_y + PLAYER_HEIGHT / 2.0;

        // Check if the ball's path intersects with the paddle
        let intersects_x = (new_x - self.x).abs() > f32::EPSILON
            && ((self.x - self.radius <= player_right && new_x + self.radius >= player_left)
                || (self.x + self.radius >= player_left && new_x - self.radius <= player_right));

        let intersects_y = (new_y - self.y).abs() > f32::EPSILON
            && ((self.y - self.radius <= player_bottom && new_y + self.radius >= player_top)
                || (self.y + self.radius >= player_top && new_y - self.radius <= player_bottom));

        intersects_x && intersects_y
    }

    // Handle the collision between the ball and a player's paddle
    fn handle_collision(&mut self, player: &Player) {
        let (player_x, player_y) = player.get_position();

        // Calculate where on the paddle the ball hit
        let collision_x = (self.x - player_x) / (PLAYER_WIDTH / 2.0);

        // Adjust angle based on where the ball hit the paddle
        // This creates more interesting gameplay by allowing players to angle their shots
        let angle = collision_x * std::f32::consts::PI / 3.0; // Max angle: 60 degrees

        // Set new velocity
        let current_speed = (self.dx.powi(2) + self.dy.powi(2)).sqrt();
        let new_speed = (current_speed * 1.01).min(MAX_BALL_SPEED); // Increase speed slightly, but cap it
        self.dx = new_speed * angle.sin();
        self.dy = -new_speed * angle.cos() * self.dy.signum(); // Reverse y direction

        // Ensure the ball is not stuck in the paddle
        // Move it slightly outside the paddle to prevent multiple collisions
        if self.y < player_y {
            self.y = player_y - PLAYER_HEIGHT / 2.0 - self.radius - f32::EPSILON;
        } else {
            self.y = player_y + PLAYER_HEIGHT / 2.0 + self.radius + f32::EPSILON;
        }
    }

    // Handle bouncing off walls (left and right sides of the screen)
    pub fn bounce_off_wall(&mut self) {
        if self.x <= self.radius || self.x >= 1.0 - self.radius {
            self.dx = -self.dx;
        }
        if self.y <= self.radius || self.y >= 1.0 - self.radius {
            self.dy = -self.dy;
        }
    }

    // Check if the ball is colliding with any wall
    pub fn collides_with_wall(&self) -> bool {
        self.x <= self.radius
            || self.x >= 1.0 - self.radius
            || self.y <= self.radius
            || self.y >= 1.0 - self.radius
    }

    // Determine which wall the ball is colliding with
    pub fn which_wall(&self) -> &str {
        if self.y <= self.radius {
            "top"
        } else if self.y >= 1.0 - self.radius {
            "bottom"
        } else if self.x <= self.radius {
            "left"
        } else if self.x >= 1.0 - self.radius {
            "right"
        } else {
            "none"
        }
    }

    pub fn reset_velocity(&mut self, serve_to_player: u8) {
        let mut rng = rand::thread_rng();

        // Randomize horizontal direction slightly
        let angle = rng.gen_range(-std::f32::consts::PI / 6.0..std::f32::consts::PI / 6.0);

        // Reset velocity with initial speed and new direction
        self.dx = INITIAL_BALL_SPEED * angle.sin();
        self.dy = if serve_to_player == 1 {
            INITIAL_BALL_SPEED * angle.cos()
        } else {
            -INITIAL_BALL_SPEED * angle.cos()
        };
    }
}
