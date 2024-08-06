use macroquad::prelude::{draw_rectangle, Color};

pub struct PixelText {
    text: String,
    position: (f32, f32),
    pixel_size: f32,
    color: Color,
    highlight_color: Color,
}

impl PixelText {
    pub fn new(
        text: &str,
        x: f32,
        y: f32,
        pixel_size: f32,
        color: Color,
        highlight_color: Color,
    ) -> Self {
        PixelText {
            text: text.to_string(),
            position: (x, y),
            pixel_size,
            color,
            highlight_color,
        }
    }

    pub fn draw(&self, selected: bool) {
        let (x, y) = self.position;
        let color = if selected {
            self.highlight_color
        } else {
            self.color
        };

        for (char_idx, ch) in self.text.chars().enumerate() {
            let char_pixels = get_char_pixels(ch);
            for (row, &pixel_row) in char_pixels.iter().enumerate() {
                for (col, &pixel) in pixel_row.iter().enumerate() {
                    if pixel {
                        draw_rectangle(
                            x + (char_idx as f32 * 6.0 + col as f32) * self.pixel_size,
                            y + (row as f32 * self.pixel_size),
                            self.pixel_size,
                            self.pixel_size,
                            color,
                        );
                    }
                }
            }
        }
    }
}

// Helper function to get pixel representation of characters
fn get_char_pixels(ch: char) -> [[bool; 5]; 7] {
    match ch {
        'A' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'E' => [
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ],
        'G' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [true, false, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'I' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, true, true, true, true],
        ],
        'J' => [
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [false, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'M' => [
            [true, false, false, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'N' => [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ],
        'O' => [
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ],
        'T' => [
            [true, true, true, true, true],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ],
        'X' => [
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, false, true, false],
            [true, false, false, false, true],
        ],
        _ => [[false; 5]; 7], // Default to empty for unknown characters
    }
}
