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
    JumpAddr {
        address: u16,
    },

    // 2nnn - CALL addr - Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the current PC
    // on the top of the stack. The PC is then set to nnn.
    Call {
        address: u16,
    },

    // 3xkk - SE Vx, byte - Skip next instruction if Vx = kk.
    SkipIfRegisterEqualImmediate {
        vx: Register,
        value: u8,
    },

    // 4xkk - SNE Vx, byte - Skip next instruction if Vx != kk.
    SkipIfRegisterNotEqualImmediate {
        vx: Register,
        value: u8,
    },

    // 5xy0 - SE Vx, Vy - Skip next instruction if Vx = Vy.
    SkipIfRegistersEqual {
        vx: Register,
        vy: Register,
    },

    // 6xkk - LD Vx, byte - The interpreter puts the value kk into register Vx.
    LoadImmediate {
        vx: Register,
        value: u8,
    },

    // 7xkk - ADD Vx, byte - Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    AddImmediate {
        vx: Register,
        value: u8,
    },

    // 8xy0 - LD Vx, Vy - Set Vx = Vy.
    SetVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy1 - OR Vx, Vy - Set Vx = Vx OR Vy.
    OrVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy2 - AND Vx, Vy - Set Vx = Vx AND Vy.
    AndVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy3 - XOR Vx, Vy - Set Vx = Vx XOR Vy.
    XOrVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy4 - ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
    // The values of Vx and Vy are added together.
    // If the result is greater than 8 bits (> 255), VF is set to 1, otherwise 0.
    // Only the lowest 8 bits of the result are kept and stored in Vx.
    AddVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy5 - SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the result is stored in Vx.
    SubVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xy6 - SHR Vx {, Vy} - Set Vx = Vx SHR 1.
    ShrVx {
        vx: Register,
    },

    // 8xy7 - SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
    SubnVxVy {
        vx: Register,
        vy: Register,
    },

    // 8xyE - SHL Vx {, Vy} - Set Vx = Vx SHL 1.
    ShlVx {
        vx: Register,
    },

    // 9xy0 - SNE Vx, Vy - Skip next instruction if Vx != Vy.
    SkipIfRegistersNotEqual {
        vx: Register,
        vy: Register,
    },

    // Annn - LD I, addr - Set I = nnn.
    LoadIndex {
        address: u16,
    },

    // Bnnn - JP V0, addr - Jump to location nnn + V0.
    JumpV0 {
        address: u16,
    },

    // Cxkk - RND Vx, byte - Set Vx = random byte AND kk.
    RandomAndImmediate {
        vx: Register,
        value: u8,
    },

    // Dxyn - DRW Vx, Vy, nibble - Display n-byte sprite starting at memory
    // location I at (Vx, Vy), set VF = collision.
    Draw {
        vx: Register,
        vy: Register,
        n: u8,
    },

    // Ex9E - SKP Vx - Skip next instruction if key with the value of Vx is pressed.
    SkipIfKeyPressed {
        vx: Register,
    },

    // ExA1 - SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
    SkipIfKeyNotPressed {
        vx: Register,
    },

    // Fx07 - LD Vx, DT - Set Vx = delay timer value.
    LoadVxDelayTimer {
        vx: Register,
    },

    // Fx0A - LD Vx, K - Wait for a key press, store the value of the key in Vx.
    WaitForKeyPress {
        vx: Register,
    },

    // Fx15 - LD DT, Vx - Set delay timer = Vx.
    SetDelayTimer {
        vx: Register,
    },

    // Fx18 - LD ST, Vx - Set sound timer = Vx.
    SetSoundTimer {
        vx: Register,
    },

    // Fx1E - ADD I, Vx - Set I = I + Vx.
    AddIndex {
        vx: Register,
    },

    // Fx29 - LD F, Vx - Set I = location of sprite for digit Vx.
    LoadFontSprite {
        vx: Register,
    },

    // Fx33 - LD B, Vx - Store BCD representation of Vx in memory locations
    // I, I+1, and I+2.
    StoreBCD {
        vx: Register,
    },

    // Fx55 - LD [I], Vx - Store registers V0 through Vx in memory starting
    // at location I.
    StoreRegisters {
        vx: Register,
    },

    // Fx65 - LD Vx, [I] - Read registers V0 through Vx from memory starting
    // at location I.
    LoadRegisters {
        vx: Register,
    },
}