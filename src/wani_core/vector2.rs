//! Struct of Vector.
//!
//! Copyright (c) 2019 wanigame
//!
//! This source code is released under the MIT License
//! http://opensource.org/licenses/mit-license.php

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

pub static ZERO: Vec2<isize> = Vec2 { x: 0, y: 0 };
pub static UP: Vec2<isize> = Vec2 { x: 0, y: -1 };
pub static DOWN: Vec2<isize> = Vec2 { x: 0, y: 1 };
pub static LEFT: Vec2<isize> = Vec2 { x: -1, y: 0 };
pub static RIGHT: Vec2<isize> = Vec2 { x: 1, y: 0 };

#[derive(Clone, Copy)]
pub struct Vec2<T> {
   pub x: T,
   pub y: T,
}

impl<T> Vec2<T> {
   pub fn new(x: T, y: T) -> Self {
      Vec2 { x, y }
   }
}

impl Vec2<isize> {
   pub fn len(self) -> f32 {
      ((self.x * self.x + self.y * self.y) as f32).sqrt()
   }
}

impl From<Vec2<isize>> for Vec2<usize> {
   fn from(from: Vec2<isize>) -> Self {
      Vec2 {
         x: from.x as usize,
         y: from.y as usize,
      }
   }
}

impl<T> PartialEq for Vec2<T>
where
   T: PartialEq,
{
   fn eq(&self, other: &Self) -> bool {
      self.x == other.x && self.y == other.y
   }
}

impl<T> Add for Vec2<T>
where
   T: Add<Output = T>,
{
   type Output = Self;

   fn add(self, other: Self) -> Self {
      Vec2 {
         x: self.x + other.x,
         y: self.y + other.y,
      }
   }
}

impl<T> AddAssign for Vec2<T>
where
   T: AddAssign,
{
   fn add_assign(&mut self, rhs: Self) {
      self.x += rhs.x;
      self.y += rhs.y;
   }
}

impl<T> Div<T> for Vec2<T>
where
   T: Div<Output = T> + Copy,
{
   type Output = Self;

   fn div(self, rhs: T) -> Self {
      Vec2 {
         x: self.x / rhs,
         y: self.y / rhs,
      }
   }
}

impl<T> Mul<T> for Vec2<T>
where
   T: Mul<Output = T> + Copy,
{
   type Output = Self;

   fn mul(self, rhs: T) -> Self {
      Vec2 {
         x: self.x * rhs,
         y: self.y * rhs,
      }
   }
}

impl<T> Sub for Vec2<T>
where
   T: Sub<Output = T>,
{
   type Output = Self;

   fn sub(self, other: Self) -> Self {
      Vec2 {
         x: self.x - other.x,
         y: self.y - other.y,
      }
   }
}

impl<T> SubAssign for Vec2<T>
where
   T: SubAssign,
{
   fn sub_assign(&mut self, rhs: Self) {
      self.x -= rhs.x;
      self.y -= rhs.y;
   }
}
