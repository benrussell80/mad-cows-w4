use std::ops::Add;
use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }

    pub fn contained_within(&self, a: Position, b: Position) -> bool {
        let x_min = a.x.min(b.x);
        let y_min = a.y.min(b.y);
        let x_max = a.x.max(b.x);
        let y_max = a.y.max(b.y);

        x_min <= self.x && self.x <= x_max && y_min <= self.y && self.y <= y_max
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.y,
            self.x + rhs.y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contained_within_simple_square() {
        assert!(
            Position::new(0.5, 0.5).contained_within(
                Position::new(0., 0.),
                Position::new(1., 1.),
            )
        )
    }

    #[test]
    fn test_contained_within_rectangle() {
        assert!(
            Position::new(0.0, 0.0).contained_within(
                Position::new(-10., 10.),
                Position::new(6., -1.),
            )
        )
    }

    #[test]
    fn test_contained_within_outside() {
        assert_eq!(
            Position::new(2., 2.).contained_within(
                Position::new(0., 0.),
                Position::new(1., 1.),
            ),
            false
        )
    }
}