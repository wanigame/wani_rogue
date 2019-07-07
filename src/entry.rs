//! Entry Point with JavaScript.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::ops::Range;
use std::sync::Mutex;

use crate::wani_character::hero::Hero;
use crate::wani_core::color::Color;
use crate::wani_core::input_manager::INPUT_MANAGER;
use crate::wani_core::rect::Rect;
use crate::wani_core::vector2::Vec2;
use crate::wani_map::map_component::MapComponent;
use crate::wani_map::random_map::RandomMap;
use crate::wani_trait::game_object::GOM;

extern "C" {
    fn js_log(log: u32);
    fn js_random(max: isize) -> isize;
    fn js_draw_rect(x: isize, y: isize, w: usize, h: usize, r: u8, g: u8, b: u8, a: u8);
}

/// Log message to console in web browser.
pub fn log(log: &str) {
    unsafe {
        for c in log.chars() {
            js_log(c as u32)
        }
        js_log('\n' as u32);
    }
}

/// Generate random numbers from range.
pub fn random(range: Range<isize>) -> isize {
    unsafe { js_random(range.end - range.start) + range.start }
}

pub fn draw_rect(rect: Rect, color: Color) {
    unsafe {
        js_draw_rect(
            rect.x, rect.y, rect.w, rect.h, color.r, color.g, color.b, color.a,
        )
    }
}

lazy_static! {
    pub static ref SCREEN_SIZE: Mutex<Rect> = Mutex::new(Rect::new(0, 0, 0, 0));
}

/// Call point from Javascript.
#[no_mangle]
pub fn init(width: usize, height: usize) {
    *SCREEN_SIZE.lock().unwrap() = Rect::new(0, 0, width, height);

    let he = Hero::new(Vec2::new(0, 0));
    GOM.lock().unwrap().regist(he);

    let rm = RandomMap::new(60, 30);

    for i in &rm.map {
        let mut m = String::new();
        for j in i {
            match j {
                MapComponent::WALL => m += "\x1b[40m　",
                MapComponent::NONE => m += "\x1b[46m　",
                MapComponent::ROOM => m += "\x1b[42m　",
            }
        }
        m += "\x1b[0m";
        log(&m);
    }

    // rm.draw();
}

#[no_mangle]
pub fn key_down(key_code: usize) {
    INPUT_MANAGER.lock().unwrap().key_down(key_code);
}

#[no_mangle]
pub fn key_up(key_code: usize) {
    INPUT_MANAGER.lock().unwrap().key_up(key_code);
}

#[no_mangle]
pub fn update() {
    GOM.lock().unwrap().update();
}

#[no_mangle]
pub fn draw() {
    GOM.lock().unwrap().draw();
}
