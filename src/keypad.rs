//!keypad.rs

use core::fmt;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keypad {
    keys: [bool; 16],

}

impl Keypad {
    pub fn new() -> Self {
        Keypad {
            keys: [false; 16],
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys[key.index()]
    }

    pub fn press(&mut self, key: Key) {
        self.keys[key.index()] = true;
    }

    pub fn release(&mut self, key: Key) {
        self.keys[key.index()] = false;
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

impl TryFrom<u8> for Key {
    type Error = KeypadError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Key::K0),
            1 => Ok(Key::K1),
            2 => Ok(Key::K2),
            3 => Ok(Key::K3),
            4 => Ok(Key::K4),
            5 => Ok(Key::K5),
            6 => Ok(Key::K6),
            7 => Ok(Key::K7),
            8 => Ok(Key::K8),
            9 => Ok(Key::K9),
            10 => Ok(Key::KA),
            11 => Ok(Key::KB),
            12 => Ok(Key::KC),
            13 => Ok(Key::KD),
            14 => Ok(Key::KE),
            15 => Ok(Key::KF),
            _ => Err(KeypadError::InvalidKey(value)),
        }
    }
}

/**
 * 
 * Custom Error
 */

#[derive(Debug, PartialEq)]
pub enum KeypadError {
    InvalidKey(u8),
}

impl fmt::Display for KeypadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeypadError::InvalidKey(key) => write!(f, "invalid key {key}")
        }
    }
}

impl std::error::Error for KeypadError {}

#[cfg(test)]
mod keypad_tests {
    use crate::keypad::{Key, Keypad, KeypadError};

    #[test]
    fn is_key_pressed_test() {
        let mut keypad = Keypad::new();
        
        keypad.press(Key::K0);

        assert_eq!(true, keypad.is_pressed(Key::K0));
    }

    #[test]
    fn is_key_release_test() {
        let mut keypad = Keypad::new();
        
        keypad.press(Key::K0);
        assert_eq!(true, keypad.is_pressed(Key::K0));
        keypad.release(Key::K0);
        assert_eq!(false, keypad.is_pressed(Key::K0));
    }

    #[test]
    fn try_from_u8_returns_correct_key() {
        assert_eq!(Key::try_from(0).unwrap(), Key::K0);
        assert_eq!(Key::try_from(10).unwrap(), Key::KA);
        assert_eq!(Key::try_from(15).unwrap(), Key::KF);
    }

    #[test]
    fn try_from_u8_rejects_invalid_key() {
        assert_eq!(
            Key::try_from(16),
            Err(KeypadError::InvalidKey(16))
        );
    }
}
