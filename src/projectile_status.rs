use crate::position::Position;
use crate::vector::Vector;
use std::fmt::Debug;


#[derive(Copy, Clone, Debug)]
pub enum ProjectileStatus {
    Reset,
    Held(Position),
    Ballistic(Vector),
}

impl ProjectileStatus {
    pub fn apply_transition(&mut self, transition: ProjectileTransition) {
        match (&self, transition) {
            (Self::Reset, ProjectileTransition::Grabbed(pos)) => *self = Self::Held(pos),
            (Self::Held(held_pos), ProjectileTransition::Released(release_pos)) => *self = Self::Ballistic(Vector::between(release_pos, *held_pos)),
            (Self::Ballistic(_), ProjectileTransition::Reset) => *self = Self::Reset,
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ProjectileTransition {
    Grabbed(Position),
    Released(Position),
    Reset,
}
