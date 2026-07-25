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