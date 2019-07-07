//! Struct of playable character.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::entry::*;

use crate::wani_core::color::Color;
use crate::wani_core::input_manager::InputKey;
use crate::wani_core::input_manager::INPUT_MANAGER;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2::Vec2;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::updater::Updater;

pub struct Hero {
    position: Vec2<usize>,
}

impl Hero {
    pub fn new(position: Vec2<usize>) -> Self {
        Hero { position }
    }

    fn r#move(&mut self, direction: Vec2<isize>) {
        self.position.x = (self.position.x as isize + direction.x) as usize;
        self.position.y = (self.position.y as isize + direction.y) as usize;
    }

    pub fn getPosition(&self) -> Vec2<usize> {
        self.position
    }
}

impl Updater for Hero {
    fn update(&mut self) {
        let im = INPUT_MANAGER.lock().unwrap();
        if im.get_key(InputKey::UP) {
            self.r#move(Vec2::new(0, -1) * 32);
        }
        if im.get_key(InputKey::DOWN) {
            self.r#move(Vec2::new(0, 1) * 32);
        }
        if im.get_key(InputKey::LEFT) {
            self.r#move(Vec2::new(-1, 0) * 32);
        }
        if im.get_key(InputKey::RIGHT) {
            self.r#move(Vec2::new(1, 0) * 32);
        }
    }
}

impl Drawer for Hero {
    fn draw(&self) {
        draw_rect(
            Rect::new(self.position.x as isize, self.position.y as isize, 32, 32),
            Color::new(0x00, 0x00, 0xff, 0xff),
        );
    }
}

impl GameObject for Hero {}
