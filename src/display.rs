const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pixels: [[bool; WIDTH];HEIGHT],
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
}