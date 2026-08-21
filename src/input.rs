//! src/input.rs

use minifb::Key::{
    A, C, D, E, F,
    Key1, Key2, Key3, Key4,
    Q, R, S, V, W, X, Z,
};

use crate::{chip8::Chip8, keypad::Key::{self, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, KA, KB, KC, KD, KE, KF}};

pub struct Input;

impl Input {
    pub fn update(window: &minifb::Window, emulator: &mut Chip8) {
        for key in [
            K0, K1, K2, K3,
            K4, K5, K6, K7,
            K8, K9, KA, KB,
            KC, KD, KE, KF,
        ] {
            emulator.release_key(key);
        }

        for key in window.get_keys() {
            if let Some(chip8_key) = map_key(key) {
                emulator.press_key(chip8_key);
            }
        }
    }
}

pub fn map_key(key: minifb::Key) -> Option<Key> {
    match key {
        Key1 => Some(K1),
        Key2 => Some(K2),
        Key3 => Some(K3),
        Key4 => Some(KC),

        Q => Some(K4),
        W => Some(K5),
        E => Some(K6),
        R => Some(KD),

        A => Some(K7),
        S => Some(K8),
        D => Some(K9),
        F => Some(KE),

        Z => Some(KA),
        X => Some(K0),
        C => Some(KB),
        V => Some(KF),

        _ => None,
    }
}