//! Struct of playable character.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::any::Any;

use crate::entry::*;

use crate::game_manager::GameManager;
use crate::wani_core::camera::DRAW_OFFSET;
use crate::wani_core::color::Color;
use crate::wani_core::input_manager::InputKey;
use crate::wani_core::input_manager::INPUT_MANAGER;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::map_component::MapComponent;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::updater::Updater;

pub struct Hero {
    position: Vec2<isize>,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            position: Vec2::new(0, 0),
        }
    }

    fn r#move(&mut self, direction: Vec2<isize>) {
        self.position += direction;
    }

    pub fn teleport(&mut self, coord: &Vec2<isize>) {
        self.position = *coord;
    }
}

impl Updater for Hero {
    fn update(&mut self, gm: &GameManager) {
        let im = INPUT_MANAGER.lock().unwrap();
        let mut dir = Vec2::new(0, 0);
        if im.get_key(InputKey::UP) {
            dir += Vec2::new(0, -1) * 32;
        }
        if im.get_key(InputKey::DOWN) {
            dir += Vec2::new(0, 1) * 32;
        }
        if im.get_key(InputKey::LEFT) {
            dir += Vec2::new(-1, 0) * 32;
        }
        if im.get_key(InputKey::RIGHT) {
            dir += Vec2::new(1, 0) * 32;
        }

        let rmap = gm.get_map().lock().unwrap();
        let map = rmap.as_any().downcast_ref::<RandomMap>().unwrap();

        let mut _dir;
        _dir = Vec2::new(dir.x, 0);
        match map.get_component(&((self.position + _dir) / 32)) {
            Some(comp) => match comp {
                MapComponent::WALL => {}
                _ => {
                    self.r#move(_dir);
                }
            },
            None => {}
        }
        _dir = Vec2::new(0, dir.y);
        match map.get_component(&((self.position + _dir) / 32)) {
            Some(comp) => match comp {
                MapComponent::WALL => {}
                _ => {
                    self.r#move(_dir);
                }
            },
            None => {}
        }
    }
}

impl Drawer for Hero {
    fn draw(&self) {
        let pos = self.position + *DRAW_OFFSET.lock().unwrap();
        draw_rect(
            Rect::new(pos.x, pos.y, 32, 32),
            Color::new(0x00, 0x00, 0xff, 0xff),
        );
    }
}

impl GameObject for Hero {
    fn get_position(&self) -> Vec2<isize> {
        self.position
    }

    fn as_any(&self) -> &Any {
        self
    }
}
