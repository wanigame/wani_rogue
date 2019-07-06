//! Trait of update regularly
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

pub trait Updater {
    fn update(&mut self);
}
