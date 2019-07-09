//! Trait of game object
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::any::Any;

use crate::wani_core::vector2::Vec2;
use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::updater::Updater;

pub trait GameObject: Drawer + Updater + Send + Any {
    fn get_position(&self) -> Vec2<isize>;

    fn as_any(&self) -> &Any;
}
