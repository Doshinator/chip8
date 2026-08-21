use std::time::{Duration, Instant};

use chip8::{
    chip8::Chip8,
    input::Input,
    render::Render,
};

const CPU_HZ: u64 = 500;
const TIMER_HZ: u64 = 60;
const FRAME_HZ: u64 = 60;
const MAX_CPU_TICKS_PER_LOOP: u32 = 10;

fn main() {
    // cli arg
    let rom_path = std::env::args()
        .nth(1)
        .expect("usage: chip8 <rom>");
    
    let rom = std::fs::read(&rom_path)
        .unwrap_or_else(|e| {
            panic!("failed to read ROM '{rom_path}': {e}");
        });

    let mut emulator = Chip8::new();

    emulator
        .load_rom(&rom)
        .expect("failed to load ROM");

    let mut renderer = Render::new();

    run_loop(&mut emulator, &mut renderer);
}

fn run_loop(emulator: &mut Chip8, renderer: &mut Render) {
    let cpu_interval = Duration::from_secs_f64(1.0 / CPU_HZ as f64);
    let timer_interval = Duration::from_secs_f64(1.0 / TIMER_HZ as f64);
    let frame_interval = Duration::from_secs_f64(1.0 / FRAME_HZ as f64);

    let mut last_cpu_tick = Instant::now();
    let mut last_timer_tick = Instant::now();
    let mut last_frame = Instant::now();

    while renderer.is_open() {
        let now = Instant::now();
        let mut cpu_ticks = 0;

        // Update CHIP-8 keypad from physical keyboard.
        Input::update(renderer.window(), emulator);

        // Run CHIP-8 instructions at 500 Hz.
        while now.duration_since(last_cpu_tick) >= cpu_interval && cpu_ticks < MAX_CPU_TICKS_PER_LOOP {
            emulator
                .tick()
                .expect("CHIP-8 execution failed");

            last_cpu_tick += cpu_interval;
            cpu_ticks += 1;
        }

        // Update CHIP-8 timers at 60 Hz.
        while now.duration_since(last_timer_tick) >= timer_interval {
            emulator.tick_timers();
            last_timer_tick += timer_interval;
        }

        // Render the display at 60 FPS.
        if now.duration_since(last_frame) >= frame_interval {
            renderer.draw(emulator.display());
            last_frame += frame_interval;
        }
    }
}