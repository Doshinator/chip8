use crate::registers::Register;

pub enum Instruction {
    LoadImmediate {
        register: Register,
        value: u8,
    },

    Jump {
        address: u16,
    },

    Call {
        address: u16,
    }    
}