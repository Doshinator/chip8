use crate::registers::Register;

/**
 * Standard Chip-8 Instructions
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    // 00E0 - CLS - Clear display
    ClearDisplay,
    
    // 00EE - RET - Return from subroutine
    Return,

    // 1nnn - JP addr - Jump to a machine code routine at nnn
    Jump {
        address: u16,
    },

    // 2nnn - CALL addr - Call subroutine at nnn. The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    Call {
        address: u16,
    },

    // 6xkk - LD Vx, byte - The interpreter puts the value kk into register Vx.
    LoadImmediate {
        register: Register,
        value: u8,
    },
    
    // 7XNN - ADD Vx, byte. Set Vx = Vx + kk. Adds the value kk to the value of register Vx, then stores the result in Vx.
    AddImmediate {
        register: Register,
        value: u8,
    },

    // 8xy0 - LD Vx, Vy; Set Vx = Vy.
    SetVxVy {
        register_destination: Register,
        register_source: Register
    },

    // 8xy4 - ADD Vx, Vy; Set Vx = Vx + Vy, set VF = carry. The values of Vx and Vy are added together. 
    //        If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    AddVxVy {
        register_destination: Register,
        register_source: Register
    },
}
