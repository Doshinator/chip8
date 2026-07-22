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
}
