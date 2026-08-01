//!chip8.rs
use core::fmt;

use crate::{decode::{DecodeError, decode}, display::Display, instruction::Instruction::{self, AddImmediate, AddVxVy, AndVxVy, Call, ClearDisplay, Jump, LoadImmediate, OrVxVy, Return, SetVxVy, ShrVx, SubVxVy, XOrVxVy}, registers::{Register, RegisterError, Registers}, stack::{Stack, StackError}};

const RAM_SIZE: usize = 4096;
pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    registers: Registers,
    stack: Stack,

    display: Display,
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
            
            stack: Stack::new(),
            display: Display::new(),
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
    fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            Return => { 
                let address = self.stack.pop()?;
                self.pc = address;
                Ok(())
            },
            ClearDisplay => { 
                self.display.clear();
                Ok(())
            },
            Jump { address } => {
                self.pc = address;
                Ok(())
            },
            Call { address } => {
                self.stack.push(self.pc)?;
                self.pc = address;
                Ok(())
            },
            LoadImmediate { register, value } => {
                self.registers.set(register, value);
                Ok(())
            },
            AddImmediate { register, value} => {
                let curr_val = self.registers.get(register).wrapping_add(value);
                self.registers.set(register, curr_val);
                Ok(())
            },
            SetVxVy { vx, vy } => {
                let src_val = self.registers.get(vy);
                self.registers.set(vx, src_val);
                Ok(())
            },
            AddVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                
                let (result, overflowed) = vx_val.overflowing_add(vy_val);
                self.registers.set(vx, result);

                if overflowed {
                    self.registers.set(Register::VF, 1);
                }
                else {
                    self.registers.set(Register::VF, 0);
                }

                Ok(())
            },
            OrVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val | vy_val);

                Ok(())
            },
            AndVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val & vy_val);
                Ok(())
            },
            XOrVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                self.registers.set(vx, vx_val ^ vy_val);
                Ok(())
            },
            SubVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                let (result, borrowed) = vx_val.overflowing_sub(vy_val);
                self.registers.set(vx, result);

                if !borrowed {
                    self.registers.set(Register::VF, 1);
                }
                else {
                    self.registers.set(Register::VF, 0);
                }

                Ok(())
            },
            ShrVx { vx } => {
                let vx_val = self.registers.get(vx);

                let least_significant_bit = vx_val & 1;
                let result = vx_val >> 1;

                self.registers.set(vx, result);
                self.registers.set(Register::VF, least_significant_bit);

                Ok(())
            }
        }
    }

    pub fn tick(&mut self) -> Result<(), Chip8Error> {
        let opcode = self.fetch();
        let instruction = decode(opcode)?;
        self.execute(instruction)?;

        Ok(())
    }
}

/**
 * 
 * Custom Error
 * 
 */
#[derive(Debug)]
pub enum Chip8Error {
    Register(RegisterError),
    Stack(StackError),
    Decode(DecodeError),
}

impl From<RegisterError> for Chip8Error {
    fn from(error: RegisterError) -> Self {
        Chip8Error::Register(error)
    }
}

impl From<StackError> for Chip8Error {
    fn from(error: StackError) -> Self {
        Chip8Error::Stack(error)
    }
}

impl From<DecodeError> for Chip8Error {
    fn from(error: DecodeError) -> Self {
        Chip8Error::Decode(error)
    }
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chip8Error::Register(e) => write!(f, "register error: {e}"),
            Chip8Error::Stack(e)    => write!(f, "stack error: {e}"),
            Chip8Error::Decode(e)   => write!(f, "decode error: {e}"),
        }
    }
}

impl std::error::Error for Chip8Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Chip8Error::Register(e) => Some(e),
            Chip8Error::Stack(e)    => Some(e),
            Chip8Error::Decode(e)   => Some(e),
        }
    }
}

/**
 * 
 * CHIP8 TEST
 * 
 */

#[cfg(test)]
mod tests {
    use crate::registers::Register;
    use super::*;

    #[test]
    fn fetch_reads_two_bytes() {
        let mut cpu = Chip8::new();
        
        cpu.memory[0x200] = 0xAB;
        cpu.memory[0x201] = 0xCD;

        let opcode = cpu.fetch();

        assert_eq!(0xABCD, opcode);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn execute_load_immediate() {
        let mut cpu = Chip8::new();
        let instruction_load_immediate = Instruction::LoadImmediate { 
            register: Register::VA, 
            value: 55,
        };

        cpu.execute(instruction_load_immediate).unwrap();

        assert_eq!(
            cpu.registers.get(Register::VA),
            55
        )
    }
}

#[cfg(test)]
mod chip8_execute_tests {
    use crate::registers::Register;
    use super::*;

     #[test]
    fn execute_jump() {
        let mut cpu = Chip8::new();
    
        cpu.execute(Instruction::Jump { address: 0x234}).unwrap();

        assert_eq!(0x234, cpu.pc)
    }

    #[test]
    fn execute_call() {
        let mut cpu = Chip8::new();

        cpu.pc = 0x202;

        cpu.execute(Instruction::Call { address: 0x300 }).unwrap();

        assert_eq!(cpu.pc, 0x300);

        assert_eq!(
            cpu.stack.pop().unwrap(),
            0x202
        );
    }
    
    #[test]
    fn execute_return() {
        let mut cpu = Chip8::new();

        cpu.stack.push(0x202).unwrap();

        cpu.execute(Instruction::Return).unwrap();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn execute_add_immediate() {
        let mut cpu = Chip8::new();
        
        cpu.registers.set(Register::VA, 10);
        cpu.execute(Instruction::AddImmediate { 
            register: Register::VA, 
            value: 255 })
            .unwrap();
        
        assert_eq!(
            cpu.registers.get(Register::VA),
            9
        );
    }

    #[test]
    fn execute_add_vxvy_without_overflow() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(AddVxVy {
            vx: Register::VA,
            vy: Register::VB,
        })
        .unwrap();

        assert_eq!(30, cpu.registers.get(Register::VA));
        assert_eq!(0, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_add_vxvy_overflow() {
        let mut cpu = Chip8::new();
        let dst = Register::VA;
        let src = Register::VB;

        cpu.registers.set(dst, 255);
        cpu.registers.set(src, 10);

        cpu.execute(AddVxVy { 
            vx: Register::VA,
            vy: Register::VB 
        })
        .unwrap();
        
        assert_eq!(9, cpu.registers.get(dst));
        assert_eq!(1, cpu.registers.get(Register::VF))
    }
    
    #[test]
    fn execute_set_vx_vy() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(SetVxVy {
            vx: Register::VA,
            vy: Register::VB,
        })
        .unwrap();

        assert_eq!(20, cpu.registers.get(Register::VA));
    }

    #[test]
    fn execute_or_vx_vy() {
        let mut cpu = Chip8::new();
        
        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.execute(OrVxVy { 
            vx: Register::VA,
            vy: Register::VB,
        })
        .unwrap();

        assert_eq!(
            0b1010_1111,
            cpu.registers.get(Register::VA)
        )
    }

    #[test]
    fn execute_and_vxvy() {
        let mut cpu = Chip8::new();
        
        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.execute(AndVxVy { 
            vx: Register::VA, 
            vy: Register::VB 
        })
        .unwrap();

        assert_eq!(
            0b0000_0001,
            cpu.registers.get(Register::VA)
        );
    }

    #[test]
    fn execute_xor_vxvy() {
        let mut cpu = Chip8::new();
        
        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.execute(XOrVxVy { 
            vx: Register::VA, 
            vy: Register::VB 
        })
        .unwrap();

        assert_eq!(
            0b1010_1110,
            cpu.registers.get(Register::VA)
        );
    }

    #[test]
    fn execute_sub_vx_vy_without_borrow() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 20);
        cpu.registers.set(Register::VB, 5);

        cpu.execute(SubVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();

        assert_eq!(15, cpu.registers.get(Register::VA));
        assert_eq!(1, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_sub_vx_vy_with_borrow() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 5);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(SubVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();

        assert_eq!(241, cpu.registers.get(Register::VA));
        assert_eq!(0, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_sub_vx_vy_equal_values() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 20);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(SubVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();

        assert_eq!(0, cpu.registers.get(Register::VA));
        assert_eq!(1, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_shr_vx_vy_lsb_zero() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0b1010_0000);

        cpu.execute(ShrVx {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(
            0b0101_0000,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            0,
            cpu.registers.get(Register::VF)
        );
    }

    #[test]
    fn execute_shr_vx_vy_lsb_one() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0b1010_0001);

        cpu.execute(ShrVx {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(
            0b0101_0000,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            1,
            cpu.registers.get(Register::VF)
        );
    }
}

#[cfg(test)]
mod chip8_tick_tests {
    use crate::{display::{HEIGHT, WIDTH}, registers::Register};
    use super::*;
    
    #[test]
    fn tick_executes_instruction() {
        let mut cpu = Chip8::new();
        cpu.memory[0x200] = 0x6A;
        cpu.memory[0x201] = 0x05;

        cpu.tick().unwrap();

        assert_eq!(
            cpu.registers.get(Register::VA),
            5
        );
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn tick_executes_call() {
        let mut cpu = Chip8::new();

        // 0x200:
        // CALL 0x300
        cpu.memory[0x200] = 0x23;
        cpu.memory[0x201] = 0x00;

        cpu.tick().unwrap();

        assert_eq!(cpu.pc, 0x300);

        assert_eq!(
            cpu.stack.pop().unwrap(),
            0x202
        );
    }

    #[test]
    fn tick_executes_return() {
        let mut cpu = Chip8::new();

        cpu.stack.push(0x202).unwrap();

        // 00EE
        cpu.memory[0x200] = 0x00;
        cpu.memory[0x201] = 0xEE;

        cpu.tick().unwrap();
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn call_then_return_resumes_execution() {
        let mut cpu = Chip8::new();

        cpu.memory[0x200] = 0x23;
        cpu.memory[0x201] = 0x00; // CALL 0x300

        cpu.memory[0x300] = 0x00;
        cpu.memory[0x301] = 0xEE; // RETURN

        cpu.tick().unwrap();

        assert_eq!(cpu.pc, 0x300);

        cpu.tick().unwrap();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn tick_clears_display() {
        let mut cpu = Chip8::new();

        cpu.memory[0x200] = 0x00;
        cpu.memory[0x201] = 0xE0;


        cpu.tick().unwrap();

        assert_eq!(
            [[false; WIDTH]; HEIGHT],
            cpu.display.pixels
        )
    }

    #[test]
    fn tick_executes_add_vxvy() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 255);
        cpu.registers.set(Register::VB, 10);
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB4;

        cpu.tick().unwrap();

        assert_eq!(9, cpu.registers.get(Register::VA));
        assert_eq!(1, cpu.registers.get(Register::VF));
        assert_eq!(0x202, cpu.pc);
    }
    
    #[test]
    fn tick_executes_set_vx_vy() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 20);

        // 8AB0 = V[A] = V[B]
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB0;

        cpu.tick().unwrap();

        assert_eq!(20, cpu.registers.get(Register::VA));
        assert_eq!(20, cpu.registers.get(Register::VB));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_or_vx_vy() {
        let mut cpu = Chip8::new();
        // 8AB1 = V[A] | V[B]
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB1;

        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.tick().unwrap();

        assert_eq!(0b1010_1111, cpu.registers.get(Register::VA));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_execute_and_vxvy() {
        let mut cpu = Chip8::new();
        // 8AB2 = V[A] & V[B]
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB2;

        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.tick().unwrap();

        assert_eq!(0b0000_0001, cpu.registers.get(Register::VA));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_execute_xor_vxvy() {
        let mut cpu = Chip8::new();
        // 8AB3 = V[A] ^ V[B]
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB3;

        cpu.registers.set(Register::VA, 0b1010_0001);
        cpu.registers.set(Register::VB, 0b0000_1111);

        cpu.tick().unwrap();

        assert_eq!(0b1010_1110, cpu.registers.get(Register::VA));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_shr_vx_vy() {
        let mut cpu = Chip8::new();

        // 8AB6 = VA >>= 1, VF = least significant bit
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xB6;

        cpu.registers.set(Register::VA, 0b1010_0001);

        cpu.tick().unwrap();

        assert_eq!(0b0101_0000,cpu.registers.get(Register::VA));
        assert_eq!(1,cpu.registers.get(Register::VF));
        assert_eq!(0x202, cpu.pc);
    }
}
