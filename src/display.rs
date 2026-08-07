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
        self.pixels[width][height]
    }

    pub fn draw_sprite(
        &mut self, 
        x: u8, 
        y: u8,
        sprite: &[u8],
    ) -> bool {
        // collision = false
        // get starting pos

        // for row, value in each sprite array
        
            // for bit position (col) 0..8
                // if bit is 0
                    // continue

                // get new_x and y pos + wrap

                // if display is already ON
                    // collision = true (1 ^ 1 -> 0 = collision, therefore it's a collision)

                // xor the pixel at new_x_pos, new_y_pos

        // return -> collision
        todo!()
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