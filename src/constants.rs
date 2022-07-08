use crate::{vector::Vector, position::Position};

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BOUNCE_DAMPING_FACTOR: f32 = 0.35;
pub const FRICTION_DAMPING_FACTOR: f32 = 0.05;
pub const COLLISION_DAMPING_FACTOR: f32 = 0.05;
pub const GRAVITY: Vector = Vector::new(0., -50.0);
pub const ORIGIN: Position = Position::new(0.0, 0.0);
pub const RADIUS: f32 = 15.0;
pub const THRESHOLD_VELOCITY: f32 = 0.001;
