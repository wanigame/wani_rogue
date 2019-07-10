//! Struct of game object manager
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::collections::HashMap;
use std::sync::Mutex;

use crate::game_manager::GameManager;
use crate::wani_trait::game_object::GameObject;

lazy_static! {
    static ref GAME_OBJECT_ID: Mutex<usize> = Mutex::new(0);
}

pub struct GameObjectManager {
    list: HashMap<usize, Mutex<Box<GameObject>>>,
}

impl GameObjectManager {
    pub fn new() -> Self {
        GameObjectManager {
            list: HashMap::new(),
        }
    }

    pub fn regist<T>(&mut self, game_object: T) -> usize
    where
        T: GameObject + 'static,
    {
        let mut id = GAME_OBJECT_ID.lock().unwrap();
        *id += 1;
        self.list.insert(*id, Mutex::new(Box::new(game_object)));
        *id
    }

    pub fn get_game_object(&self, id: usize) -> &Mutex<Box<GameObject + 'static>> {
        &self.list[&id]
    }

    pub fn update(&self, gm: &GameManager) {
        for u in &self.list {
            u.1.lock().unwrap().update(gm);
        }
    }

    pub fn draw(&self) {
        for d in &self.list {
            d.1.lock().unwrap().draw();
        }
    }
}
