//! Game manager for wani_rogue
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::sync::Mutex;

use crate::entry::SCREEN_SIZE;
use crate::game_object_manager::GameObjectManager;
use crate::wani_character::hero::Hero;
use crate::wani_core::camera::Camera;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::game_object::GameObject;

lazy_static! {
    pub static ref GAME_MANAGER: Mutex<GameManager> = Mutex::new(GameManager::new());
}

pub struct GameManager {
    gom: GameObjectManager,

    main_camera: Camera,

    pub id_hero: usize,
    pub id_map: usize,
}

impl GameManager {
    pub fn new() -> Self {
        let mut gom = GameObjectManager::new();

        let map = RandomMap::new(80, 50);

        let mut hero = Hero::new();
        hero.teleport(&map.respawnable_coord());

        let id_map = gom.regist(map);
        let id_hero = gom.regist(hero);

        GameManager {
            gom,

            main_camera: Camera::new(),

            id_hero,
            id_map,
        }
    }

    pub fn update(&mut self) {
        self.gom.update(&self);

        // calculate offset
        let target = self.get_hero().lock().unwrap().get_position();
        self.main_camera.offset = SCREEN_SIZE.lock().unwrap().center() - Vec2::new(16, 16) - target;
        // move camera offset
        self.main_camera.set();
    }

    pub fn draw(&self) {
        self.gom.draw();
    }

    pub fn get_hero(&self) -> &Mutex<Box<GameObject>> {
        self.gom.get_game_object(self.id_hero).unwrap()
    }
    pub fn get_map(&self) -> &Mutex<Box<GameObject>> {
        self.gom.get_game_object(self.id_map).unwrap()
    }
}
