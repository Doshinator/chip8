//!decode.rs
use core::fmt;

use crate::{instruction::{Instruction}, registers::Register::{self}};

pub fn decode(opcode: u16) -> Result<Instruction, DecodeError> {
    let instruction = (opcode >> 12) as u8;

    match instruction {
        0 => {
            match opcode & 0x00FF {
                0xE0 => Ok(Instruction::ClearDisplay),
                0xEE => Ok(Instruction::Return),
                _ => Err(DecodeError::UnsupportedInstruction(opcode))
            }
        },
        1 => {
            let address = opcode & 0x0FFF;
            Ok(Instruction::Jump { address })
        },
        2 => {
            let address = opcode & 0x0FFF;
            Ok(Instruction::Call { address })
        },
        6 => {
            let register_index = ((opcode >> 8) as u8) & (0x0F);
            let value = (opcode & 0x00FF) as u8;

            let register = Register::from_index(register_index)
                .map_err(|_| DecodeError::UnsupportedInstruction(opcode))?;

            Ok(Instruction::LoadImmediate { register, value })
        },
        7 => {
            let register_index = ((opcode >> 8) as u8) & (0x0F);
            let register = Register::from_index(register_index)
                .map_err(|_| DecodeError::UnsupportedInstruction(opcode))?;
            let value = (opcode & 0x00FF) as u8;

            Ok(Instruction::AddImmediate { register, value })
        },
        8 => {
            let x = ((opcode >> 8) & 0x0F) as u8;
            let y = ((opcode >> 4) & 0x0F) as u8;

            match opcode & 0x000F {
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                4 => {
                    let register_destination = Register::from_index(x).expect("X register must be valid");
                    let register_source = Register::from_index(y).expect("Y register must be valid");

                    Ok(Instruction::AddVxVy { register_destination,  register_source })
                },
                _ => Err(DecodeError::UnsupportedInstruction(opcode))
            }
        }
        _ => Err(DecodeError::UnsupportedInstruction(opcode)),
    }
}

/**
 * Errors that occur while decoding opcodes
 */
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    // Opcode does not map to a CHIP-8 instruction
    UnsupportedInstruction(u16),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::UnsupportedInstruction(op) => {
                write!(f, "unsupported opcode: {op:#06X}")
            }
        }
    }
}
impl std::error::Error for DecodeError {}

/**
 * 
 * DECODE TEST
 * 
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_load_immediate() {
        let instruction = decode(0x6A05).unwrap();
        assert_eq!(
            instruction,
            Instruction::LoadImmediate {
                register: Register::VA,
                value: 5,
            }
        );
    }

    #[test]
    fn decode_jump() {
        let instruction = decode(0x1234).unwrap();
        assert_eq!(
            instruction,
            Instruction::Jump { 
                address: 0x234 
            }
        );
    }

    #[test]
    fn decode_add_immedaite() {
        let instruction = decode(0x7A55).unwrap();
        assert_eq!(
            Instruction::AddImmediate { 
                register: Register::VA, 
                value: 0x55
            },
            instruction
        )
    }

    #[test]
    fn decode_add_vxvy() {
        let instruction = decode(0x8AB4).unwrap();
        assert_eq!(
            Instruction::AddVxVy { 
                register_destination: Register::VA, 
                register_source: Register::VB 
            },
            instruction
        )
    }
}
