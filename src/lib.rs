//! This is roguelike system by WebAssembly with Rust.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

pub mod entry;
pub mod wani_character;
pub mod wani_core;
pub mod wani_map;
pub mod wani_trait;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
