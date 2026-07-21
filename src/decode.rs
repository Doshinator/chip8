use crate::{instruction::Instruction, registers::{Register::{self}, RegisterError}};

pub fn decode(opcode: u16) -> Result<Instruction, RegisterError> {
    let instruction = (opcode >> 12) as u8;
    let register_index = ((opcode >> 8) as u8) & (0x0F);
    let value = (opcode & 0x00FF) as u8;

    match instruction {
        6 => {
            let register = Register::from_index(register_index)?;
            Ok(Instruction::LoadImmediate { register, value })
        }
        _ => {
            panic!()
        }
    }
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
}
