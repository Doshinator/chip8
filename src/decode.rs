//!decode.rs
use core::fmt;

use crate::{decode::DecodeError::UnsupportedInstruction, instruction::Instruction, registers::Register::{self}};

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
            let (vx, vy) = decode_xy_register(opcode)?;
            match opcode & 0x000F {
                0 => Ok(Instruction::SetVxVy { vx, vy }),
                1 => Ok(Instruction::OrVxVy { vx, vy }),
                2 => Ok(Instruction::AndVxVy { vx, vy }),
                3 => todo!(),
                4 => Ok(Instruction::AddVxVy { vx,  vy }),
                _ => Err(DecodeError::UnsupportedInstruction(opcode))
            }
        }
        _ => Err(DecodeError::UnsupportedInstruction(opcode)),
    }
}

fn decode_xy_register(opcode: u16) -> Result<(Register, Register), DecodeError> {
    let x = ((opcode >> 8) & 0x0F) as u8;
    let y = ((opcode >> 4) & 0x0F) as u8;
    let vx = Register::from_index(x)
        .map_err(|_| UnsupportedInstruction(opcode))?;
    let vy = Register::from_index(y)
        .map_err(|_| UnsupportedInstruction(opcode))?;

    Ok((vx, vy))
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
    fn decode_add_vx_vy() {
        let instruction = decode(0x8AB4).unwrap();
        assert_eq!(
            Instruction::AddVxVy { 
                vx: Register::VA, 
                vy: Register::VB 
            },
            instruction
        )
    }

    #[test]
    fn decode_set_vx_vy() {
        let instruction = decode(0x8AB0).unwrap();
        assert_eq!(
            Instruction::SetVxVy {
                vx: Register::VA,
                vy: Register::VB,
            },
            instruction
        );
    }

    #[test]
    fn decode_or_vx_vy() {
        let instruction = decode(0x8AB1).unwrap();
        assert_eq!(
            Instruction::OrVxVy { 
                vx: Register::VA,
                vy: Register::VB,
            },
            instruction
        );
    }
}
