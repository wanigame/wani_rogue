//! Trait of game object
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::updater::Updater;

pub trait GameObject: Drawer + Updater + Send {}

pub struct GameObjectManager<'a> {
    list: Vec<&'a mut GameObject>,
}

impl<'a> GameObjectManager<'a> {
    pub fn new() -> Self {
        GameObjectManager { list: Vec::new() }
    }

    pub fn regist<T>(&mut self, game_object: &'a mut T)
    where
        T: GameObject,
    {
        self.list.push(game_object);
    }

    pub fn update(&mut self) {
        for u in &mut self.list {
            u.update();
        }
    }

    pub fn draw(&self) {
        for d in &self.list {
            d.draw();
        }
    }
}
