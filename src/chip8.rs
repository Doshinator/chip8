//!chip8.rs
use core::fmt;

use rand::RngExt;

use crate::{decode::{DecodeError, decode}, display::Display, font::FONT, instruction::Instruction::{self, AddImmediate, AddIndex, AddVxVy, AndVxVy, Call, ClearDisplay, Draw, JumpAddr, JumpV0, LoadFontSprite, LoadImmediate, LoadIndex, LoadRegisters, LoadVxDelayTimer, OrVxVy, RandomAndImmediate, Return, SetDelayTimer, SetSoundTimer, SetVxVy, ShlVx, ShrVx, SkipIfKeyNotPressed, SkipIfKeyPressed, SkipIfRegisterEqualImmediate, SkipIfRegisterNotEqualImmediate, SkipIfRegistersEqual, SkipIfRegistersNotEqual, StoreBCD, StoreRegisters, SubVxVy, SubnVxVy, WaitForKeyPress, XOrVxVy}, keypad::{Key, Keypad, KeypadError}, registers::{Register, RegisterError, Registers}, stack::{Stack, StackError}, timer::Timer};

const RAM_SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;

pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    registers: Registers,
    stack: Stack,

    display: Display,
    keypad: Keypad,

    delay_timer: Timer,
    sound_timer: Timer,
    
    waiting_for_key: Option<Register>,

    pc: u16,
    index: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self {
            memory: [0; RAM_SIZE],
            registers: Registers::new(),
            
            stack: Stack::new(),
            display: Display::new(),
            keypad: Keypad::new(),

            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            
            waiting_for_key: None,
            
            pc: 0x200,
            index: 0,
        };

        chip8.memory[..FONT.len()].copy_from_slice(&FONT);

        chip8
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), Chip8Error> {
        
        for (i, value) in rom.iter().enumerate() {
            self.memory[PROGRAM_START + i] = *value;
        }
        Ok(())
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
            JumpAddr { address } => {
                self.pc = address;
                Ok(())
            },
            Call { address } => {
                self.stack.push(self.pc)?;
                self.pc = address;
                Ok(())
            },
            SkipIfRegisterEqualImmediate { vx, value } => {
                if value == self.registers.get(vx) {
                    self.pc += 2;
                }
                Ok(())
            },
            SkipIfRegisterNotEqualImmediate { vx, value } => {
                if value != self.registers.get(vx) {
                    self.pc += 2;
                }
                Ok(())
            },
            SkipIfRegistersEqual { vx, vy } => {
                if self.registers.get(vx) == self.registers.get(vy) {
                    self.pc += 2;
                }
                Ok(())
            },
            LoadImmediate { vx, value } => {
                self.registers.set(vx, value);
                Ok(())
            },
            AddImmediate { vx, value} => {
                let curr_val = self.registers.get(vx).wrapping_add(value);
                self.registers.set(vx, curr_val);
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

                self.registers.set(Register::VF, if !borrowed { 1 } else { 0 });
                Ok(())
            },
            ShrVx { vx } => {
                let vx_val = self.registers.get(vx);

                let least_significant_bit = vx_val & 1;
                let result = vx_val >> 1;

                self.registers.set(vx, result);
                self.registers.set(Register::VF, least_significant_bit);

                Ok(())
            },
            SubnVxVy { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);
                let (result, borrowed) = vy_val.overflowing_sub(vx_val);

                self.registers.set(vx, result);

                self.registers.set(Register::VF, if !borrowed { 1 } else { 0 });
                Ok(())
            },
            ShlVx { vx } => {
                let vx_val = self.registers.get(vx);

                let most_significant_bit = vx_val >> 7;
                let result = vx_val << 1;

                self.registers.set(vx, result);
                self.registers.set(Register::VF, most_significant_bit);

                Ok(())
            },
            SkipIfRegistersNotEqual { vx, vy } => {
                let vx_val = self.registers.get(vx);
                let vy_val = self.registers.get(vy);

                if vx_val != vy_val {
                    self.pc += 2;
                }

                Ok(())
            },
            LoadIndex { address } => {
                self.index = address;
                Ok(())
            },
            JumpV0 { address } => {
                self.pc = address + (self.registers.get(Register::V0) as u16);
                Ok(())
            },
            RandomAndImmediate { vx, value } => {
                let mut rng = rand::rng();
                let random_byte: u8 = rng.random();
                self.registers.set(vx, random_byte & value);
                Ok(())
            },
            Draw { vx, vy, n } => {
                let x = self.registers.get(vx);
                let y = self.registers.get(vy);

                let start = self.index as usize;
                let end = start + n as usize;

                let sprite = &self.memory[start..end];

                let collision = self.display.draw_sprite(x, y, sprite);

                self.registers.set(
                    Register::VF,
                    if collision { 1 } else { 0 }
                );

                Ok(())
            },
            SkipIfKeyPressed { vx } => {
                let x = self.registers.get(vx);
                let key = Key::try_from(x)?;
                
                if self.keypad.is_pressed(key) {
                    self.pc += 2;
                }

                Ok(())
            },
            SkipIfKeyNotPressed { vx } => {
                let x = self.registers.get(vx);
                let key = Key::try_from(x)?;

                if !self.keypad.is_pressed(key) {
                    self.pc += 2;
                }

                Ok(())
            },
            LoadVxDelayTimer { vx } => {
                let value = self.delay_timer.get();
                self.registers.set(vx, value);
                
                Ok(())
            },
            WaitForKeyPress { vx } => {
                self.waiting_for_key = Some(vx);
                Ok(())
            },
            SetDelayTimer { vx} => {
                let x = self.registers.get(vx);
                self.delay_timer.set(x);
                Ok(())
            },
            SetSoundTimer { vx } => {
                let x = self.registers.get(vx);
                self.sound_timer.set(x);
                Ok(())
            },
            AddIndex { vx } => {
                let x = self.registers.get(vx);
                self.index = self.index.wrapping_add(x as u16);

                Ok(())
            },
            LoadFontSprite { vx } => {
                let x = self.registers.get(vx);
                self.index = x as u16 * 5;

                Ok(())
            },
            StoreBCD { vx } => {
                let x = self.registers.get(vx);

                // extract the place-th of the u8 value. 123 = 1-hundreds 2-tens 3-ones
                let hundreds = x / 100;
                let tens = (x / 10) % 10;
                let ones = x % 10;
                let i = self.index as usize;

                self.memory[i] = hundreds;
                self.memory[i + 1] = tens;
                self.memory[i + 2] = ones;
                Ok(())
            },
            StoreRegisters { vx } => {
                // stores into memory values starting from register v0 -> vX where vX is one of 16 registers, starting at address index.
                for i in 0..=Register::index(&vx) {
                    let address = self.index as usize + i;
                    let register = Register::from_index(i as u8)?;

                    self.memory[address] = self.registers.get(register);
                }
                Ok(())
            },
            LoadRegisters { vx } => {
                for i in 0..=Register::index(&vx) {
                    let address = self.index as usize + i;
                    let register = Register::from_index(i as u8)?;
                    let value = self.memory[address];

                    self.registers.set(register, value);
                }
                Ok(())                
            }
        }
    }

    pub fn tick(&mut self) -> Result<(), Chip8Error> {
        if let Some(vx) = self.waiting_for_key {
            if let Some(key) = self.keypad.pressed_key() {
                self.registers.set(vx, key.index() as u8);
                self.waiting_for_key = None;
            }

            return Ok(())
        }

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
    Keypad(KeypadError),
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

impl From<KeypadError> for Chip8Error {
    fn from(error: KeypadError) -> Self {
        Chip8Error::Keypad(error)
    }
}

impl fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chip8Error::Register(e) => write!(f, "register error: {e}"),
            Chip8Error::Stack(e)    => write!(f, "stack error: {e}"),
            Chip8Error::Decode(e)   => write!(f, "decode error: {e}"),
            Chip8Error::Keypad(e) => write!(f, "keypad error: {e}"),
        }
    }
}

impl std::error::Error for Chip8Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Chip8Error::Register(e) => Some(e),
            Chip8Error::Stack(e) => Some(e),
            Chip8Error::Decode(e) => Some(e),
            Chip8Error::Keypad(e) => Some(e),
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
            vx: Register::VA, 
            value: 55,
        };

        cpu.execute(instruction_load_immediate).unwrap();

        assert_eq!(
            cpu.registers.get(Register::VA),
            55
        )
    }

    #[test]
    fn new_loads_font_into_memory() {
        let cpu = Chip8::new();

        assert_eq!(0xF0, cpu.memory[0x000]);
        assert_eq!(0x90, cpu.memory[0x001]);
        assert_eq!(0x90, cpu.memory[0x002]);
        assert_eq!(0x90, cpu.memory[0x003]);
        assert_eq!(0xF0, cpu.memory[0x004]);
    }

    #[test]
    fn load_rom_test() {
        let mut cpu = Chip8::new();
        let rom: &[u8] = &[0x60, 0x0A, 0x61, 0x05];
        
        cpu.load_rom(rom).unwrap();

        for (i, value) in rom.iter().enumerate() {
            println!("{}", cpu.memory[PROGRAM_START + i]);
            assert_eq!(*value, cpu.memory[PROGRAM_START + i]);
        }
    }
}

#[cfg(test)]
mod chip8_execute_tests {
    use crate::{keypad::Key::*, registers::Register::*};
    use super::*;

     #[test]
    fn execute_jump() {
        let mut cpu = Chip8::new();
    
        cpu.execute(Instruction::JumpAddr { address: 0x234}).unwrap();

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
            vx: Register::VA, 
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
    fn execute_shr_vx_lsb_zero() {
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
    fn execute_shr_vx_lsb_one() {
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

    #[test]
    fn execute_subn_vx_vy_without_borrow() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 5);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(SubnVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();
      
        assert_eq!(15, cpu.registers.get(Register::VA));
        assert_eq!(1, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_subn_vx_vy_with_borrow() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 20);
        cpu.registers.set(Register::VB, 5);

        cpu.execute(SubnVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();
      
        assert_eq!(241, cpu.registers.get(Register::VA));
        assert_eq!(0, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_subn_vx_vy_equal_values() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 20);
        cpu.registers.set(Register::VB, 20);

        cpu.execute(SubnVxVy {
            vx: Register::VA,
            vy: Register::VB,
        }).unwrap();

        assert_eq!(0, cpu.registers.get(Register::VA));
        assert_eq!(1, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_shl_vx_msb_zero() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0b0101_0001);

        cpu.execute(ShlVx {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(
            0b1010_0010,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            0,
            cpu.registers.get(Register::VF)
        );
    }

    #[test]
    fn execute_shl_vx_msb_one() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0b1010_0001);

        cpu.execute(ShlVx {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(
            0b0100_0010,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            1,
            cpu.registers.get(Register::VF)
        );
    }

    #[test]
    fn execute_sne_vx_vy_skips_when_not_equal() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 20);
        cpu.pc = 0x202;

        cpu.execute(SkipIfRegistersNotEqual {
            vx: Register::VA,
            vy: Register::VB,
        })
        .unwrap();

        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn execute_sne_vx_vy_does_not_skip_when_equal() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 10);
        cpu.pc = 0x202;

        cpu.execute(SkipIfRegistersNotEqual {
            vx: Register::VA,
            vy: Register::VB,
        })
        .unwrap();

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn execute_loadindex() {
        let mut cpu = Chip8::new();
        
        cpu.execute(LoadIndex { 
            address: 0x123 
        })
        .unwrap();

        assert_eq!(0x123, cpu.index);
    }

    #[test]
    fn execute_jump_v0() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::V0, 0x10);

        cpu.execute(Instruction::JumpV0 { address: 0x300 }).unwrap();

        assert_eq!(0x310, cpu.pc);
    }

    #[test]
    fn execute_random_and_immediate_respects_mask() {
        let mut cpu = Chip8::new();

        let mask = 0b0000_1111;

        for _ in 0..100 {
            cpu.execute(Instruction::RandomAndImmediate {
                vx: Register::VA,
                value: mask,
            }).unwrap();

            let result = cpu.registers.get(Register::VA);

            assert_eq!(result & !mask, 0);
        }
    }

    #[test]
    fn execute_random_and_immediate_with_zero_mask() {
        let mut cpu = Chip8::new();

        cpu.execute(RandomAndImmediate {
            vx: Register::VA,
            value: 0x00,
        }).unwrap();

        assert_eq!(0, cpu.registers.get(Register::VA));
    }

    #[test]
    fn execute_draw_sprite() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;
        cpu.memory[0x300] = 0b1111_0000;
        cpu.memory[0x301] = 0b1001_0000;

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 5);

        cpu.execute(Draw {
            vx: Register::VA,
            vy: Register::VB,
            n: 2,
        }).unwrap();

        // First sprite row: ****
        assert!(cpu.display.is_on(10, 5));
        assert!(cpu.display.is_on(11, 5));
        assert!(cpu.display.is_on(12, 5));
        assert!(cpu.display.is_on(13, 5));

        // Second sprite row: *  *
        assert!(cpu.display.is_on(10, 6));
        assert!(cpu.display.is_on(13, 6));

        assert_eq!(0, cpu.registers.get(Register::VF));
    }

    #[test]
    fn execute_draw_sprite_sets_vf_on_collision() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;
        cpu.memory[0x300] = 0b1000_0000;

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 5);

        // First draw: pixel is off, so no collision.
        cpu.execute(Draw {
            vx: Register::VA,
            vy: Register::VB,
            n: 1,
        }).unwrap();

        assert_eq!(0, cpu.registers.get(Register::VF));

        // Second draw: same pixel is already on.
        cpu.execute(Draw {
            vx: Register::VA,
            vy: Register::VB,
            n: 1,
        }).unwrap();

        assert_eq!(1, cpu.registers.get(Register::VF));

        // XOR means drawing it again erased it.
        assert!(!cpu.display.is_on(5, 10));
    }

    #[test]
    fn execute_skip_key_pressed_true() {
        let mut cpu = Chip8::new();

        cpu.pc = 0x200;
        cpu.registers.set(Register::VA, 10);
        cpu.keypad.press(KA);

        cpu.execute(Instruction::SkipIfKeyPressed {
            vx: Register::VA
        })
        .unwrap();

        assert_eq!(true, cpu.keypad.is_pressed(KA));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn execute_skip_key_pressed_false() {
        let mut cpu = Chip8::new();

        cpu.pc = 0x200;

        cpu.execute(Instruction::SkipIfKeyPressed {
            vx: Register::VA
        })
        .unwrap();

        assert_eq!(false, cpu.keypad.is_pressed(KA));
        assert_eq!(0x200, cpu.pc);
    }

    #[test]
    fn execute_skip_key_not_pressed_true() {
        let mut cpu = Chip8::new();

        cpu.pc = 0x200;

        cpu.execute(Instruction::SkipIfKeyNotPressed {
            vx: Register::VA
        })
        .unwrap();

        assert_eq!(true, !cpu.keypad.is_pressed(KA));
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn execute_skip_key_not_pressed_false() {
        let mut cpu = Chip8::new();

        cpu.pc = 0x200;
        cpu.registers.set(Register::VA, 10);
        cpu.keypad.press(KA);

        cpu.execute(Instruction::SkipIfKeyNotPressed {
            vx: Register::VA
        })
        .unwrap();

        assert_eq!(true, cpu.keypad.is_pressed(KA));
        assert_eq!(0x200, cpu.pc);
    }

    #[test]
    fn execute_load_vx_delay_timer() {
        let mut cpu = Chip8::new();

        cpu.delay_timer.set(42);

        cpu.execute(Instruction::LoadVxDelayTimer {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(42, cpu.registers.get(Register::VA));
    }
    
    #[test]
    fn execute_wait_for_key_true() {
        let mut cpu = Chip8::new();
        
        cpu.execute(WaitForKeyPress { 
            vx: VA 
        })
        .unwrap();

        assert_eq!(Some(VA), cpu.waiting_for_key);
    }

    #[test]
    fn execute_wait_for_key_none() {
        let cpu = Chip8::new();

        assert_eq!(None, cpu.waiting_for_key);
    }

    #[test]
    fn execute_set_delay_timer() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 42);

        cpu.execute(Instruction::SetDelayTimer {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(42, cpu.delay_timer.get());
    }

    #[test]
    fn execute_set_sound_timer() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 42);

        cpu.execute(Instruction::SetSoundTimer {
            vx: Register::VA,
        }).unwrap();

        assert_eq!(42, cpu.sound_timer.get());
    }

    #[test]
    fn execute_add_index() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;
        cpu.registers.set(Register::VA, 0x20);

        cpu.execute(Instruction::AddIndex { 
            vx: Register::VA 
        })
        .unwrap();

        assert_eq!(0x320, cpu.index);
    }

    #[test]
    fn execute_add_index_wraps() {
        let mut cpu = Chip8::new();

        cpu.index = 0xFFFF;
        cpu.registers.set(Register::VA, 1);

        cpu.execute(Instruction::AddIndex {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(0x0000, cpu.index);
    }

    #[test]
    fn execute_load_font_sprite() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0xA);

        cpu.execute(Instruction::LoadFontSprite {
            vx: Register::VA,
        })
        .unwrap();

        assert_eq!(0x32, cpu.index);
    }

    #[test]
    fn execute_store_bcd() {
        let mut cpu = Chip8::new();
        cpu.index = 0x300;
        let i = cpu.index as usize;
        
        cpu.registers.set(Register::V0, 123);

        cpu.execute(Instruction::StoreBCD {
            vx: Register::V0
        })
        .unwrap();

        assert_eq!(1, cpu.memory[i]);
        assert_eq!(2, cpu.memory[i + 1]);
        assert_eq!(3, cpu.memory[i + 2]);
    }

    #[test]
    fn execute_storebcd_with_leading_zeroes() {
        let mut cpu = Chip8::new();
        cpu.index = 0x300;
        let i = cpu.index as usize;

        cpu.registers.set(Register::V0, 7);

        cpu.execute(Instruction::StoreBCD {
            vx: Register::V0
        })
        .unwrap();

        assert_eq!(0, cpu.memory[i]);
        assert_eq!(0, cpu.memory[i + 1]);
        assert_eq!(7, cpu.memory[i + 2]);
    }

    #[test]
    fn execute_store_registers() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;

        cpu.registers.set(Register::V0, 10);
        cpu.registers.set(Register::V1, 20);
        cpu.registers.set(Register::V2, 30);
        cpu.registers.set(Register::V3, 40);

        cpu.execute(Instruction::StoreRegisters {
            vx: Register::V3,
        }).unwrap();

        assert_eq!(10, cpu.memory[0x300]);
        assert_eq!(20, cpu.memory[0x301]);
        assert_eq!(30, cpu.memory[0x302]);
        assert_eq!(40, cpu.memory[0x303]);
        assert_eq!(0, cpu.memory[0x304]);
    }

    #[test]
    fn execute_load_registers() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;

        cpu.memory[0x300] = 10;
        cpu.memory[0x301] = 20;
        cpu.memory[0x302] = 30;
        cpu.memory[0x303] = 40;

        cpu.execute(Instruction::LoadRegisters {
            vx: Register::V3,
        }).unwrap();

        assert_eq!(10, cpu.registers.get(Register::V0));
        assert_eq!(20, cpu.registers.get(Register::V1));
        assert_eq!(30, cpu.registers.get(Register::V2));
        assert_eq!(40, cpu.registers.get(Register::V3));

        // V4 should not have been touched.
        assert_eq!(0, cpu.registers.get(Register::V4));
    }
}

#[cfg(test)]
mod chip8_tick_tests {
use crate::{display::{HEIGHT, WIDTH}, registers::Register::{self, VA}};
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
    fn tick_executes_shr_vx() {
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

    #[test]
    fn tick_executes_shl_vx_msb_zero() {
        let mut cpu = Chip8::new();

        // 8ABE = VA <<= 1, VF = most significant bit
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xBE;

        cpu.registers.set(Register::VA, 0b0101_0001);

        cpu.tick().unwrap();

        assert_eq!(
            0b1010_0010,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            0,
            cpu.registers.get(Register::VF)
        );

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_shl_vx_msb_one() {
        let mut cpu = Chip8::new();

        // 8ABE = VA <<= 1, VF = most significant bit
        cpu.memory[0x200] = 0x8A;
        cpu.memory[0x201] = 0xBE;

        cpu.registers.set(Register::VA, 0b1010_0001);

        cpu.tick().unwrap();

        assert_eq!(
            0b0100_0010,
            cpu.registers.get(Register::VA)
        );

        assert_eq!(
            1,
            cpu.registers.get(Register::VF)
        );

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_sne_vx_vy() {
        let mut cpu = Chip8::new();

        // 9AB0 = skip next instruction if VA != VB
        cpu.memory[0x200] = 0x9A;
        cpu.memory[0x201] = 0xB0;

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 20);

        cpu.tick().unwrap();

        // fetch advances to 0x202, then 9xy0 skips another instruction
        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn tick_executes_loadindex() {
        let mut cpu = Chip8::new();

        cpu.memory[0x200] = 0xA1;
        cpu.memory[0x201] = 0x23;

        cpu.tick().unwrap();

        assert_eq!(0x123, cpu.index);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_jump_v0() {
        let mut cpu = Chip8::new();

        // B300 = jump to 0x300 + V0
        cpu.memory[0x200] = 0xB3;
        cpu.memory[0x201] = 0x00;

        cpu.registers.set(Register::V0, 0x10);

        cpu.tick().unwrap();

        assert_eq!(0x310, cpu.pc);
    }

    #[test]
    fn tick_executes_draw_sprite() {
        let mut cpu = Chip8::new();

        // DAB2 = draw 2-byte sprite at (VA, VB)
        cpu.memory[0x200] = 0xDA;
        cpu.memory[0x201] = 0xB2;

        cpu.index = 0x300;
        cpu.memory[0x300] = 0b1111_0000;
        cpu.memory[0x301] = 0b1001_0000;

        cpu.registers.set(Register::VA, 10);
        cpu.registers.set(Register::VB, 5);

        cpu.tick().unwrap();

        assert!(cpu.display.is_on(10, 5));
        assert!(cpu.display.is_on(11, 5));
        assert!(cpu.display.is_on(12, 5));
        assert!(cpu.display.is_on(13, 5));

        assert!(cpu.display.is_on(10, 6));
        assert!(cpu.display.is_on(13, 6));

        assert_eq!(0x202, cpu.pc);
        assert_eq!(0, cpu.registers.get(Register::VF));
    }

    #[test]
    fn tick_executes_skip_if_pressed() {
        let mut cpu = Chip8::new();

        // EX9E = skip next instruction if V0 key is pressed
        cpu.memory[0x200] = 0xE0;
        cpu.memory[0x201] = 0x9E;

        cpu.registers.set(Register::V0, 0x5);
        cpu.keypad.press(Key::K5);

        cpu.tick().unwrap();

        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn tick_executes_skip_if_pressed_when_not_pressed() {
        let mut cpu = Chip8::new();

        // EX9E
        cpu.memory[0x200] = 0xE0;
        cpu.memory[0x201] = 0x9E;

        cpu.registers.set(Register::V0, 0x5);

        cpu.tick().unwrap();

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_skip_if_not_pressed() {
        let mut cpu = Chip8::new();

        // EXA1 = skip next instruction if V0 key is NOT pressed
        cpu.memory[0x200] = 0xE0;
        cpu.memory[0x201] = 0xA1;

        cpu.registers.set(Register::V0, 0x5);

        cpu.tick().unwrap();

        assert_eq!(0x204, cpu.pc);
    }

    #[test]
    fn tick_executes_skip_if_not_pressed_when_pressed() {
        let mut cpu = Chip8::new();

        // EXA1
        cpu.memory[0x200] = 0xE0;
        cpu.memory[0x201] = 0xA1;

        cpu.registers.set(Register::V0, 0x5);
        cpu.keypad.press(Key::K5);

        cpu.tick().unwrap();

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_load_vx_delay_timer() {
        let mut cpu = Chip8::new();
        
        cpu.delay_timer.set(42);
        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x07;

        cpu.tick().unwrap();

        assert_eq!(42, cpu.registers.get(Register::VA));
        assert_eq!(42, cpu.delay_timer.get());
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_wait_for_key_press() {
        let mut cpu = Chip8::new();

        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x0A;

        // First tick executes FX0A and enters waiting state.
        cpu.tick().unwrap();

        assert_eq!(cpu.waiting_for_key, Some(VA));

        // FX0A was fetched, so PC advanced once.
        assert_eq!(cpu.pc, 0x202);

        // No key has been pressed yet.
        cpu.tick().unwrap();

        // CPU is waiting, so PC must not advance.
        assert_eq!(cpu.pc, 0x202);
        assert_eq!(cpu.registers.get(VA), 0);

        // Now press K5.
        cpu.keypad.press(Key::K5);

        cpu.tick().unwrap();

        assert_eq!(Key::K5.index(), cpu.registers.get(VA) as usize);
        // Waiting state should be cleared.
        assert_eq!(cpu.waiting_for_key, None);

        // PC still hasn't advanced because the waiting logic does not fetch
        // another instruction on the key-press tick.
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn tick_executes_set_delay_timer() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 42);

        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x15;

        cpu.tick().unwrap();

        assert_eq!(42, cpu.delay_timer.get());
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_set_sound_timer() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 42);

        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x18;

        cpu.tick().unwrap();

        assert_eq!(42, cpu.sound_timer.get());
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_add_index() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;
        cpu.registers.set(Register::VA, 0x20);

        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x1E;

        cpu.tick().unwrap();

        assert_eq!(0x320, cpu.index);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_load_font_sprite() {
        let mut cpu = Chip8::new();

        cpu.registers.set(Register::VA, 0xA);

        // FX29 — LD F, VA
        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x29;

        cpu.tick().unwrap();

        assert_eq!(0x032, cpu.index);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_store_bcd() {
        let mut cpu = Chip8::new();

        // FA33 = FX33 with X = A
        // Store BCD representation of VA at I, I+1, I+2.
        cpu.memory[0x200] = 0xFA;
        cpu.memory[0x201] = 0x33;

        cpu.index = 0x300;
        cpu.registers.set(Register::VA, 123);

        cpu.tick().unwrap();

        assert_eq!(1, cpu.memory[0x300]);
        assert_eq!(2, cpu.memory[0x301]);
        assert_eq!(3, cpu.memory[0x302]);
        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_store_registers() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;

        cpu.registers.set(Register::V0, 10);
        cpu.registers.set(Register::V1, 20);
        cpu.registers.set(Register::V2, 30);
        cpu.registers.set(Register::V3, 40);

        // F355 = LD [I], V3
        cpu.memory[0x200] = 0xF3;
        cpu.memory[0x201] = 0x55;

        cpu.tick().unwrap();

        assert_eq!(10, cpu.memory[0x300]);
        assert_eq!(20, cpu.memory[0x301]);
        assert_eq!(30, cpu.memory[0x302]);
        assert_eq!(40, cpu.memory[0x303]);

        assert_eq!(0x202, cpu.pc);
    }

    #[test]
    fn tick_executes_load_registers() {
        let mut cpu = Chip8::new();

        cpu.index = 0x300;

        cpu.memory[0x300] = 10;
        cpu.memory[0x301] = 20;
        cpu.memory[0x302] = 30;
        cpu.memory[0x303] = 40;

        // FX65 — LD V0..V3, [I]
        cpu.memory[0x200] = 0xF3;
        cpu.memory[0x201] = 0x65;

        cpu.tick().unwrap();

        assert_eq!(10, cpu.registers.get(Register::V0));
        assert_eq!(20, cpu.registers.get(Register::V1));
        assert_eq!(30, cpu.registers.get(Register::V2));
        assert_eq!(40, cpu.registers.get(Register::V3));

        assert_eq!(0x202, cpu.pc);
    }
}
