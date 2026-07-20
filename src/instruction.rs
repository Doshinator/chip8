use crate::registers::Register;

/**
 * Standard Chip-8 Instructions
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    // Clear display - 00E0
    ClearDisplay,
    
    // Return from subroutine - 00EE
    Return,

    // Jump to a machine code routine at nnn
    Jump {
        address: u16,
    },

    LoadImmediate {
        register: Register,
        value: u8,
    },

    Call {
        address: u16,
    }    
}