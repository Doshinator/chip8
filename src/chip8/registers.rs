const NUM_REGS: usize = 16;

struct Registers {
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
enum Register {
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
    fn index(&self) -> usize {
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
}