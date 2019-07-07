//! Struct of Camera.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::sync::Mutex;

use crate::wani_core::vector2::Vec2;

lazy_static! {
    pub static ref DRAW_OFFSET: Mutex<Vec2<isize>> = Mutex::new(Vec2::new(0, 0));
}

pub struct Camera {
    pub offset: Vec2<isize>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            offset: Vec2::new(0, 0),
        }
    }

    pub fn set(&self) {
        *DRAW_OFFSET.lock().unwrap() = self.offset;
    }
}
