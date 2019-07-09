//! Trait of update regularly
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::game_manager::GameManager;

pub trait Updater {
    fn update(&mut self, gm: &GameManager);
}
