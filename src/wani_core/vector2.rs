//! Struct of Vector.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

pub static ZERO: Vec2 = Vec2 { x: 0, y: 0 };
pub static UP: Vec2 = Vec2 { x: 0, y: -1 };
pub static DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub static LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub static RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Clone, Copy)]
pub struct Vec2 {
   pub x: isize,
   pub y: isize,
}

impl Vec2 {
   pub fn new(x: isize, y: isize) -> Self {
      Vec2 { x, y }
   }

   pub fn len(self) -> f32 {
      ((self.x * self.x + self.y * self.y) as f32).sqrt()
   }
}

impl PartialEq for Vec2 {
   fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
   }
}

impl Add for Vec2 {
   type Output = Self;

   fn add(self, other: Self) -> Self {
      Vec2 {
         x: self.x + other.x,
         y: self.y + other.y,
      }
   }
}

impl AddAssign for Vec2 {
   fn add_assign(&mut self, rhs: Self) {
      self.x += rhs.x;
      self.y += rhs.y;
   }
}

impl Div<isize> for Vec2 {
   type Output = Self;

   fn div(self, rhs: isize) -> Self {
      Vec2 {
         x: self.x / rhs,
         y: self.y / rhs,
      }
   }
}

impl Mul<isize> for Vec2 {
   type Output = Self;

   fn mul(self, rhs: isize) -> Self {
      Vec2 {
         x: self.x * rhs,
         y: self.y * rhs,
      }
   }
}

impl Sub for Vec2 {
   type Output = Self;

   fn sub(self, other: Self) -> Self {
      Vec2 {
         x: self.x - other.x,
         y: self.y - other.y,
      }
   }
}

impl SubAssign for Vec2 {
   fn sub_assign(&mut self, rhs: Self) {
      self.x -= rhs.x;
      self.y -= rhs.y;
   }
}
