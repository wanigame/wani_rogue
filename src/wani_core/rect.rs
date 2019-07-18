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

   pub fn left(&self) -> isize {
      self.x
   }
   pub fn right(&self) -> isize {
      self.x + self.w as isize
   }
   pub fn top(&self) -> isize {
      self.y
   }
   pub fn bottom(&self) -> isize {
      self.y + self.h as isize
   }

   pub fn slide(&mut self, vec: &Vec2) {
      self.x += vec.x;
      self.y += vec.y;
   }

   pub fn center(&self) -> Vec2 {
      Vec2 {
         x: (self.x + self.w as isize) / 2,
         y: (self.y + self.h as isize) / 2,
      }
   }

   pub fn contains(&self, point: Vec2) -> bool {
      self.left() <= point.x
         && point.x <= self.right()
         && self.top() <= point.y
         && point.y <= self.bottom()
   }

   pub fn contains_rect(&self, rect: Rect) -> bool {
      self.left() <= rect.left() && rect.left() <= self.right()
         || self.left() <= rect.right() && rect.right() <= self.right()
         || self.top() <= rect.top() && rect.top() <= self.bottom()
         || self.top() <= rect.bottom() && rect.bottom() <= self.bottom()
   }
}
