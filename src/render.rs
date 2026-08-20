//!render.rs
use crate::display::Display;
use minifb::{Window, WindowOptions};

const MINIFB_WIDTH: usize = 640;
const MINIFB_HEIGHT: usize = 320;


pub struct Render {
    window: Window,
    buffer: Vec<u32>,
}

impl Render {
    pub fn new() -> Self {
        let window = Window::new(
            "CHIP-8",
            MINIFB_WIDTH,
            MINIFB_HEIGHT,
            WindowOptions::default(),
        )
        .expect("failed to create window");

        let buffer = vec![0; MINIFB_WIDTH * MINIFB_HEIGHT];

        Self { window, buffer }
    }

    pub fn draw(&mut self, display: &Display) {
        render(display, &mut self.buffer);

        self.window
            .update_with_buffer(
                &self.buffer,
                MINIFB_WIDTH,
                MINIFB_HEIGHT,
            )
            .expect("failed to update window");
    }
    
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn pressed_keys(&self) -> Vec<minifb::Key> {
        self.window.get_keys()
    }
}


fn render(display: &Display, buffer: &mut [u32]) {
    buffer.fill(0);

    for y in 0..32 {
        for x in 0..64 {
            if display.is_on(x, y) {
                let screen_x = x * 10;
                let screen_y = y * 10;

                for pixel_y in 0..10 {
                    for pixel_x in 0..10 {
                        let actual_x = screen_x + pixel_x;
                        let actual_y = screen_y + pixel_y;

                        let index = actual_y * MINIFB_WIDTH + actual_x;
                        
                        buffer[index] = 0xFFFFFF;
                    }
                }
            }
        }
    }
}
