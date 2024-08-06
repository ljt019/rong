use macroquad::prelude::*;

const PIXEL_SIZE: f32 = 6.0;
const LETTER_WIDTH: usize = 5;
const LETTER_HEIGHT: usize = 7;
const LETTER_SPACING: f32 = 2.0;
const WAVE_WIDTH: usize = 3; // Width of the shimmer wave

// Define the pixel patterns for each letter
const LETTER_R: [[bool; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [true, true, true, true, false],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, true, true, true, false],
    [true, false, true, false, false],
    [true, false, false, true, false],
    [true, false, false, false, true],
];

const LETTER_O: [[bool; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [false, true, true, true, false],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

const LETTER_N: [[bool; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [true, false, false, false, true],
    [true, true, false, false, true],
    [true, false, true, false, true],
    [true, false, false, true, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
];

const LETTER_G: [[bool; LETTER_WIDTH]; LETTER_HEIGHT] = [
    [false, true, true, true, false],
    [true, false, false, false, true],
    [true, false, false, false, false],
    [true, false, true, true, true],
    [true, false, false, false, true],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

struct PixelWave {
    position: f32,
    speed: f32,
}

impl PixelWave {
    fn new() -> Self {
        Self {
            position: 0.0,
            speed: 20.0, // Adjust this value to change wave speed
        }
    }

    fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
        if self.position >= 100.0 {
            self.position -= 100.0;
        }
    }
}

pub struct TitleText {
    text: Vec<([[bool; LETTER_WIDTH]; LETTER_HEIGHT], char)>,
    wave: PixelWave,
    pub position: Vec2,
}

impl TitleText {
    pub fn new(text: &str, x: f32, y: f32) -> Self {
        let text: Vec<_> = text
            .to_uppercase()
            .chars()
            .filter_map(|c| match c {
                'R' => Some((LETTER_R, 'R')),
                'O' => Some((LETTER_O, 'O')),
                'N' => Some((LETTER_N, 'N')),
                'G' => Some((LETTER_G, 'G')),
                _ => None,
            })
            .collect();

        Self {
            text,
            wave: PixelWave::new(),
            position: Vec2::new(x, y),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.wave.update(dt);
    }

    pub fn draw(&self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let total_width = self.text.len() as f32
            * ((LETTER_WIDTH as f32 * (PIXEL_SIZE + 1.0) - 1.0) + LETTER_SPACING * PIXEL_SIZE);
        let start_x = (screen_width - total_width) / 2.0;
        let start_y = (screen_height - (LETTER_HEIGHT as f32 * (PIXEL_SIZE + 1.0))) / 2.0;

        let mut x_offset = 0.0;
        for (index, (letter, _)) in self.text.iter().enumerate() {
            self.draw_pixel_letter(letter, start_x + x_offset, start_y, index);
            x_offset +=
                (LETTER_WIDTH as f32 * (PIXEL_SIZE + 1.0) - 1.0) + LETTER_SPACING * PIXEL_SIZE;
        }
    }

    fn draw_pixel_letter(
        &self,
        letter: &[[bool; LETTER_WIDTH]; LETTER_HEIGHT],
        x: f32,
        y: f32,
        letter_index: usize,
    ) {
        let total_pixels = self.text.len() * LETTER_WIDTH;
        let wave_start = (self.wave.position / 100.0 * total_pixels as f32) as usize;

        //let wave_color = Color::new(1.0, 0.5, 0.0, 1.0); // Orange

        // Bright Whitish yellow/orange
        let wave_color: Color = Color::new(1.0, 0.9, 0.7, 1.0);

        let base_color = Color::new(0.5, 0.25, 0.0, 1.0); // Dim orange/brown

        for (row, pixels) in letter.iter().enumerate() {
            for (col, &pixel) in pixels.iter().enumerate() {
                if pixel {
                    let pixel_index = letter_index * LETTER_WIDTH + col;
                    let distance_from_wave =
                        (pixel_index + total_pixels - wave_start) % total_pixels;

                    let color = if distance_from_wave < WAVE_WIDTH {
                        let t = distance_from_wave as f32 / WAVE_WIDTH as f32;
                        let brightness = (1.0 - t) * 0.5 + 0.5; // Adjust this for different shimmer intensity
                        Color::new(
                            wave_color.r,
                            wave_color.g * brightness,
                            wave_color.b * brightness,
                            wave_color.a,
                        )
                    } else {
                        base_color
                    };

                    draw_rectangle(
                        x + (col as f32 * (PIXEL_SIZE + 1.0)),
                        y + (row as f32 * (PIXEL_SIZE + 1.0)),
                        PIXEL_SIZE,
                        PIXEL_SIZE,
                        color,
                    );
                }
            }
        }
    }
}
