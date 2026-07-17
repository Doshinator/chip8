use crate::{instruction::Instruction, registers::Register::{self, V5}};

pub fn decode(opcode: u16) -> Instruction {
    // extract instruction
    let instruction = (opcode >> 12) as u8;

    // extract register
    let register = ((opcode >> 8) as u8) & (0x0F);

    // extract value
    let value = (opcode & 0x00FF) as u8;

    match instruction {
        6 => {
            let register = match register {
                5 => V5,
                _ => panic!()  
            };
            Instruction::LoadImmediate {
                register, 
                value 
            }
        }
        _ => {
            panic!()
        }
    }
}

