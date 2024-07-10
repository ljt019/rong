// The ball has a x and y position.

pub struct Ball {
    x: u8,
    y: u8,
}

impl Ball {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn set_position(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }

    fn get_position(&self) -> (u8, u8) {
        (self.x, self.y)
    }
}
