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
        self.x += self.dx;
        self.y += self.dy;

        // Keep the ball within bounds
        self.x = self.x.max(0.0).min(9.0);
        self.y = self.y.max(0.0).min(9.0);
    }

    pub fn bounce_off_player(&mut self, player: &Player) {
        let (player_x, player_y) = player.get_position();

        if self.x < player_x {
            self.dx = -1.0;
        } else if self.x > player_x {
            self.dx = 1.0;
        }

        if self.y < player_y {
            self.dy = -1.0;
        } else if self.y > player_y {
            self.dy = 1.0;
        }
    }

    pub fn bounce_off_wall(&mut self) {
        if self.x <= 0.0 || self.x >= 9.0 {
            self.dx = -self.dx;
        }

        if self.y <= 0.0 || self.y >= 9.0 {
            self.dy = -self.dy;
        }
    }

    pub fn collides_with_player(&self, player: &Player) -> bool {
        let (player_x, player_y) = player.get_position();

        (self.x - player_x).abs() < 1.0 && (self.y - player_y).abs() < 1.0
    }

    pub fn collides_with_wall(&self) -> bool {
        self.x <= 0.0 || self.x >= 9.0 || self.y <= 0.0 || self.y >= 9.0
    }
}
