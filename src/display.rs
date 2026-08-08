//!display.rs

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub pixels: [[bool; WIDTH];HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT]
    }

    pub fn is_on(&self, width: usize, height: usize) -> bool {
        self.pixels[height][width]
    }

    pub fn draw_sprite(
        &mut self, 
        x: u8, 
        y: u8,
        sprite: &[u8],
    ) -> bool {
        let mut collision = false;
        let start_x = x as usize;
        let start_y = y as usize;
        
        for (sprite_row, sprite_byte) in sprite.iter().enumerate() {
            for sprite_col in 0..8 {
                let bit = (sprite_byte >> (7 - sprite_col)) & 1;

                if bit == 0 { continue }
                
                let pos_x = (start_x + sprite_col) % WIDTH;
                let pos_y = (start_y + sprite_row) % HEIGHT;

                if self.pixels[pos_y][pos_x] {
                    collision = true;
                }
                
                self.pixels[pos_y][pos_x] ^= true;
            }
        }

        collision
    }
}


/**
 * 
 * TESTS Display
 * 
 */
#[cfg(test)]
mod display_tests {
    use crate::display::{Display, HEIGHT, WIDTH};

    #[test]
    fn clear_display() {
        let mut display = Display::new();
        display.pixels = [[true; WIDTH]; HEIGHT];
        display.clear();

        assert_eq!(
            [[false; WIDTH]; HEIGHT],
            display.pixels
        )
    }
}