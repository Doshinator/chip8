

const RAM_SIZE: usize = 4096;
pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    registers: Registers,
    stack: Stack,

    display: Display,
    keypad: Keypad,

    display_timer: Timer,
    sound_timer: Timer,

    pc: u16,
    index: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
            registers: Registers::new(),
            
            stack: Stack::new(),
            display: Display::new(),
            keypad: Keypad::new(),

            display_timer: Time::new(),
            sound_timer: Time::new(),
            
            pc: 0x200,
            index: 0,
        }
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        let op = (high_byte << 8) | low_byte;
        
        self.pc += 2;

        op
    }
}