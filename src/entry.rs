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
    unsafe { js_random(range.start.abs() + range.end.abs()) + range.start }
}

/// Call point from Javascript.
#[no_mangle]
pub fn init() {
    let rm = RandomMap::new(10, 10);

    for i in rm.map {
        let mut m = String::new();
        for j in i {
            match j {
                MapComponent::WALL => m += "壁",
                MapComponent::NONE => m += "　",
            }
        }
        log(&m);
    }
}
