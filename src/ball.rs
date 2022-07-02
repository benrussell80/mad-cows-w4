use crate::position::Position;
use crate::vector::Vector;
use crate::drawable::Drawable;
use crate::go_ballistic;
use crate::wasm4::*;

#[derive(Copy, Clone, Debug)]
pub struct Ball {
    position: Position,
    velocity: Vector,
    radius: f32,
}

impl Ball {
    pub const fn new(position: Position, velocity: Vector, radius: f32) -> Self {
        Self {
            position,
            velocity,
            radius
        }
    }

    pub fn mv(&mut self, acceleration: Vector, time_seconds: f32) {
        (self.position, self.velocity) = go_ballistic(self.position, self.velocity, acceleration, time_seconds);
    }

    pub const fn get_velocity(&self) -> Vector {
        self.velocity
    }

    pub const fn get_position(&self) -> Position {
        self.position
    }
}

impl Drawable for Ball {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_height(&self) -> f32 {
        self.radius
    }

    fn get_width(&self) -> f32 {
        self.radius
    }

    fn draw(&self, (x, y): (i32, i32), scale: f32) {
        let radius = (self.radius * scale) as u32;
        oval(x, y, radius, radius);
    }
}