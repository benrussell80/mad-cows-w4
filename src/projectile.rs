use crate::position::Position;
use crate::vector::Vector;
use std::fmt::Debug;


#[derive(Copy, Clone, Debug)]
pub struct Projectile<T: Copy + Clone + Debug> {
    pub mass: f32,
    pub position: Position,
    pub velocity: Vector,
    pub object: T // another field to how the drawing parts (for total inelastic collisions)
}

#[derive(Copy, Clone, Debug)]
pub enum CollisionKind {
    Elastic,
    Inelastic,
    TotalInelastic
}

impl<T: Copy + Clone + Debug> Projectile<T> {
    pub fn collide(&mut self, other: &mut Self, how: CollisionKind) {
        match how {
            CollisionKind::Elastic => {
                // the two objects' momentums "switch"
                let svxi = self.velocity.x;
                let spxi = svxi * self.mass;
                let svyi = self.velocity.y;
                let spyi = svyi * self.mass;

                let ovxi = other.velocity.x;
                let opxi = ovxi * other.mass;
                let ovyi = other.velocity.y;
                let opyi = ovyi * other.mass;

                self.velocity.x = -opxi / self.mass;
                self.velocity.y = -opyi / self.mass;

                other.velocity.x = -spxi / other.mass;
                other.velocity.y = -spyi / other.mass;
            },
            _ => unimplemented!()
        }
    }

    // pub fn mv(&mut self, acceleration: Vector, time_seconds: f32) {
    //     (self.position, self.velocity) = go_ballistic(self.position, self.velocity, acceleration, time_seconds);
    // }

    pub fn ballistic_step(&mut self, acceleration: Vector, time_step_seconds: f32) {
        self.position.x += time_step_seconds * self.velocity.x;
        self.position.y += time_step_seconds * self.velocity.y;
        self.velocity.x += time_step_seconds * acceleration.x;
        self.velocity.y += time_step_seconds * acceleration.y;
    }
}