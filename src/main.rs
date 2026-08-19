use std::time::{Duration, Instant};

use chip8::{display::Display, render::Render};
const CPU_HZ: u64 = 500;
const TIMER_HZ: u64 = 60;

fn main() {
    // println!("CHIP-8 emulator");

    // let rom = std::fs::read("roms/test.ch8")
    //     .expect("failed to read ROM");

    // let cpu_interval = Duration::from_secs_f64(1.0 / CPU_HZ as f64);
    // let timer_interval = Duration::from_secs_f64(1.0 / TIMER_HZ as f64);
    
    // let mut last_cpu_tick = Instant::now();
    // let mut last_timer_tick = Instant::now();

    // let mut emulator = Chip8::new();

    // emulator
    //     .load_rom(&rom)
    //     .expect("failed to load ROM");

    // loop {
    //     let now = Instant::now();

    //     if now.duration_since(last_cpu_tick) >= cpu_interval {
    //         emulator.tick().expect("CHIP-8 execution failed");
    //         last_cpu_tick = now;
    //     }
        
    //     if now.duration_since(last_timer_tick) >= timer_interval {
    //         emulator.tick_timers();
    //         last_timer_tick = now;
    //     }

    //     emulator.tick().expect("CHIP-8 execution failed");
    // }

    let mut display = Display::new();
    display.set_pixel(0, 0, true);
    display.set_pixel(63, 31, true);
    display.set_pixel(32, 16, true);
    
    let mut renderer = Render::new();

    while renderer.is_open() {
        renderer.draw(&display);
    }
    
}


