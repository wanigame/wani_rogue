//! Struct of playable character.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::wani_core::vector2::Vec2;

pub struct Hero {
    position: Vec2<usize>,
}

impl Hero {
    pub fn new(position: Vec2<usize>) -> Self {
        Hero { position }
    }

    pub fn r#move(&mut self, direction: Vec2<usize>) {
        self.position += direction;
    }
}
