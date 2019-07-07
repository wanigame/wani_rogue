//! Trait of game object
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::sync::Mutex;

use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::updater::Updater;

lazy_static! {
    pub static ref GOM: Mutex<GameObjectManager> = Mutex::new(GameObjectManager::new());
}

pub trait GameObject: Drawer + Updater + Send {}

pub struct GameObjectManager {
    list: Vec<Mutex<Box<GameObject>>>,
}

impl GameObjectManager {
    pub fn new() -> Self {
        GameObjectManager { list: Vec::new() }
    }

    pub fn regist<T>(&mut self, game_object: T)
    where
        T: GameObject + 'static,
    {
        self.list.push(Mutex::new(Box::new(game_object)));
    }

    pub fn update(&mut self) {
        for u in &self.list {
            u.lock().unwrap().update();
        }
    }

    pub fn draw(&self) {
        for d in &self.list {
            d.lock().unwrap().draw();
        }
    }
}
