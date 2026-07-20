use crate::{decode::decode, instruction::Instruction::{self, LoadImmediate}, registers::{RegisterError, Registers}, stack::Stack};


const RAM_SIZE: usize = 4096;
pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    registers: Registers,
    // stack: Stack,

    // display: Display,
    // keypad: Keypad,

    // display_timer: Timer,
    // sound_timer: Timer,

    pc: u16,
    index: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
            registers: Registers::new(),
            
            // stack: Stack::new(),
            // display: Display::new(),
            // keypad: Keypad::new(),

            // display_timer: Time::new(),
            // sound_timer: Time::new(),
            
            pc: 0x200,
            index: 0,
        }
    }

    // fetch op code
    fn fetch(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        let op = (high_byte << 8) | low_byte;
        
        self.pc += 2;

        op
    }

    // execute instructions
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            LoadImmediate { register, value } => {
                self.registers.set(register, value);
            },
            _ => panic!()
        }
    }

    pub fn tick(&mut self) -> Result<(), RegisterError> {
        let opcode = self.fetch();
        let instruction = decode(opcode)?;
        self.execute(instruction);

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::chip8::Chip8;

    #[test]
    fn fetch_reads_two_bytes() {
        let mut cpu = Chip8::new();
        
        cpu.memory[0x200] = 0xAB;
        cpu.memory[0x201] = 0xCD;

        let opcode = cpu.fetch();

        assert_eq!(0xABCD, opcode);
        assert_eq!(cpu.pc, 0x202);
    }
}