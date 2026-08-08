//!keypad.rs
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keypad {
    keys: [bool; 16],

}

impl Keypad {
    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys[key.index()]
    }

    pub fn press(key: Key) {
        todo!()
    }

    pub fn release(key: Key) {
        todo!()
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

impl Key {
    pub fn index(self) -> usize {
        match self {
            Key::K0 => 0,
            Key::K1 => 1,
            Key::K2 => 2,
            Key::K3 => 3,
            Key::K4 => 4,
            Key::K5 => 5,
            Key::K6 => 6,
            Key::K7 => 7,
            Key::K8 => 8,
            Key::K9 => 9,
            Key::KA => 10,
            Key::KB => 11,
            Key::KC => 12,
            Key::KD => 13,
            Key::KE => 14,
            Key::KF => 15,
        }
    }
}

/**
 * 
 * Custom Error for keypad
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeypadError {
    UnsupportedKey(usize)
}

impl fmt::Display for KeypadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeypadError::UnsupportedKey(key) => {
                write!(f, "unsupported key: {key}")
            }
        }
    }
}
impl std::error::Error for KeypadError {}
