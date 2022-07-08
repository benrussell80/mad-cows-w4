use std::ops::{Add, Div, Mul, Sub, Neg};
use crate::position::Position;
use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn between(p0: Position, p1: Position) -> Self {
        Self::new(p1.x - p0.x, p1.y - p0.y)
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, other: Vector) -> Self::Output {
        Self::Output {
            x: other.x * self,
            y: other.y * self,
        }
    }
}

impl Mul for Vector {
    type Output = f32;
    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl Div<f32> for Vector {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}