//! Game manager for wani_rogue
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::sync::Mutex;

use crate::game_object_manager::GameObjectManager;
use crate::wani_character::hero::Hero;
use crate::wani_core::camera::Camera;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::updater::Updater;

lazy_static! {
    pub static ref GAME_MANAGER: Mutex<GameManager<'static>> = Mutex::new(GameManager::new());
}

pub struct GameManager<'a> {
    gom: GameObjectManager<'a>,

    main_camera: Camera,

    hero: Hero,
    map: RandomMap,
}

impl<'a> GameManager<'a> {
    pub fn new() -> Self {
        GameManager {
            gom: GameObjectManager::new(),

            main_camera: Camera::new(),

            hero: Hero::new(),
            map: RandomMap::new(80, 50),
        }
    }

    pub fn update(&mut self) {
        self.gom.update();

        self.map.update();
        self.hero.update();

        // move camera offset
        self.main_camera.offset = self.hero.get_offset();
        self.main_camera.set();
    }

    pub fn draw(&self) {
        self.gom.draw();

        self.map.draw();
        self.hero.draw();
    }
}
