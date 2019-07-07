//! Game manager for wani_rogue
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::wani_character::hero::Hero;
use crate::wani_core::camera::Camera;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::game_object::GameObject;
use crate::wani_trait::game_object::GOM;
use crate::wani_trait::updater::Updater;

pub struct GameManager {
    main_camera: Camera,
    hero: Hero,
    map: RandomMap,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            main_camera: Camera::new(),
            hero: Hero::new(Vec2::new(0, 0)),
            map: RandomMap::new(50, 50),
        }
    }

    pub fn update(&mut self) {
        GOM.lock().unwrap().update();

        // move camera offset
        self.main_camera.offset = self.hero.get_offset();
    }

    pub fn draw(&self) {
        GOM.lock().unwrap().draw();
    }
}
