use crate::{instruction::{Instruction}, registers::Register::{self}};

pub fn decode(opcode: u16) -> Result<Instruction, DecodeError> {
    let instruction = (opcode >> 12) as u8;

    match instruction {
        0 => {
            let low_byte = opcode & 0x00FF;

            match low_byte {
                0x00E0 => Ok(Instruction::ClearDisplay),
                0x00EE => Ok(Instruction::Return),
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
}
