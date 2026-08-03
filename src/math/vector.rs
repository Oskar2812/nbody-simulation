#![allow(dead_code)]

use std::ops::{Add, Mul, Sub, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn magnitude_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalised(&self) -> Vec2 {
        let len: f64 = self.magnitude();
        if len < 1e-9 {
            return *self
        }

        Vec2 { x: self.x / len, y: self.y / len}
    }

    pub fn dot(&self, other: Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 { x: self * vector.x, y: self * vector.y }
    }
}