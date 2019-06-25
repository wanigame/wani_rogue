//! Entry Point with JavaScript.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::ops::Range;

use super::wani_map::map_component::MapComponent;
use super::wani_map::random_map::RandomMap;

extern "C" {
    fn js_log(log: u32);
    fn js_random(max: isize) -> isize;
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

/// Call point from Javascript.
#[no_mangle]
pub fn init() {
    let rm = RandomMap::new(60, 30);

    for i in rm.map {
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
}
