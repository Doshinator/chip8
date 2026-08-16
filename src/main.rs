use chip8::chip8::Chip8;

fn main() {
    println!("CHIP-8 emulator");
    let rom = std::fs::read("roms/test.ch8")
        .expect("failed to read ROM");

    let mut emulator = Chip8::new();

    emulator
        .load_rom(&rom)
        .expect("failed to open ROM");

    loop {
        emulator.tick().expect("CHIP-8 execution failed");
    }
}
