//! Struct of manager of input.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::sync::Mutex;

lazy_static! {
    pub static ref INPUT_MANAGER: Mutex<InputManager> = Mutex::new(InputManager::new());
}

pub struct InputManager {
    inputs: Vec<usize>,
}

pub enum InputKey {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager { inputs: Vec::new() }
    }

    pub fn key_down(&mut self, key_code: usize) {
        self.inputs.push(key_code);
    }

    pub fn key_up(&mut self, key_code: usize) {
        self.inputs.retain(|&e| e != key_code);
    }

    pub fn get_key(&self, key: InputKey) -> bool {
        match key {
            InputKey::UP => self.inputs.contains(&38) || self.inputs.contains(&87),
            InputKey::DOWN => self.inputs.contains(&40) || self.inputs.contains(&83),
            InputKey::LEFT => self.inputs.contains(&37) || self.inputs.contains(&65),
            InputKey::RIGHT => self.inputs.contains(&39) || self.inputs.contains(&68),
        }
    }
}
