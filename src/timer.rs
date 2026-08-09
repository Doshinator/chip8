pub struct Timer {
    time: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer { time: 0 }
    }

    pub fn set(&mut self, time: u8) {
        self.time = time;
    }

    pub fn get(&self) -> u8 {
        self.time        
    }

    pub fn tick(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        }
    }
}