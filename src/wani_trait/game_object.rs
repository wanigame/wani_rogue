//! Trait of game object
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::wani_trait::drawer::Drawer;
use crate::wani_trait::updater::Updater;

pub trait GameObject: Drawer + Updater + Send {}
