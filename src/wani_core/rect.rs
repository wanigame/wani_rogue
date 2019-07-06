//! Struct of Rect.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use crate::wani_core::vector2::Vec2;

#[derive(Clone, Copy)]
pub struct Rect {
   pub x: isize,
   pub y: isize,
   pub w: usize,
   pub h: usize,
}

impl Rect {
   pub fn new(x: isize, y: isize, w: usize, h: usize) -> Self {
      Rect { x, y, w, h }
   }

   pub fn slide(&mut self, vec: &Vec2<isize>) {
      self.x += vec.x;
      self.y += vec.y;
   }
}
