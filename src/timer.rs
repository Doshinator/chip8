pub struct Timer {
    value: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer { value: 0 }
    }

    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get(&self) -> u8 {
        self.value        
    }

    pub fn tick(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}

#[cfg(test)]
mod time_tests {
    use crate::timer::Timer;

    #[test]
    fn new_timer_starts_at_zero() {
        let timer = Timer::new();

        assert_eq!(0, timer.get());
    }

    #[test]
    fn tick_does_not_go_below_zero() {
        let mut timer = Timer::new();

        timer.tick();

        assert_eq!(0, timer.get());
    }
    
    #[test]
    fn tick_decrements_time() {
        let mut timer = Timer::new();
        
        timer.set(60);
        timer.tick();

        assert_eq!(59, timer.get());
    }

    #[test]
    fn set_updates_timer() {
        let mut timer = Timer::new();

        timer.set(42);

        assert_eq!(42, timer.get());
    }
}