const NUM_REGS: usize = 16;

pub struct Registers {
    values: [u8; NUM_REGS],
}

impl Registers {
    fn new() -> Self {
        Self {
            values: [0; NUM_REGS],
        }
    }

    fn get(&self, register: Register) -> u8 {
        self.values[register.index()]
    }
    
    fn set(
        &mut self,
        register: Register,
        val: u8,
    ) {
        self.values[register.index()] = val;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl Register {
    pub fn index(&self) -> usize {
        match self {
            Register::V0 => 0,
            Register::V1 => 1,
            Register::V2 => 2,
            Register::V3 => 3,
            Register::V4 => 4,
            Register::V5 => 5,
            Register::V6 => 6,
            Register::V7 => 7,
            Register::V8 => 8,
            Register::V9 => 9,
            Register::VA => 10,
            Register::VB => 11,
            Register::VC => 12,
            Register::VD => 13,
            Register::VE => 14,
            Register::VF => 15,
        }
    }
    
    pub fn from_index(index: u8) -> Result<Register, RegisterError> {
        match index {
            0 => Ok(Register::V0),
            1 => Ok(Register::V1),
            2 => Ok(Register::V2),
            3 => Ok(Register::V3),
            4 => Ok(Register::V4),
            5 => Ok(Register::V5),
            6 => Ok(Register::V6),
            7 => Ok(Register::V7),
            8 => Ok(Register::V8),
            9 => Ok(Register::V9),
            10 => Ok(Register::VA),
            11 => Ok(Register::VB),
            12 => Ok(Register::VC),
            13 => Ok(Register::VD),
            14 => Ok(Register::VE),
            15 => Ok(Register::VF),
            _ => Err(RegisterError::InvalidIndex(index)),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterError {
    InvalidIndex(u8),
}