#![allow(dead_code)]
#[cfg(feature = "buddy-alloc")]
mod alloc;

mod wasm4;
use wasm4::*;

mod vector;
use vector::Vector;

mod position;
use position::Position;

mod frame;
use frame::Frame;

mod drawable;

mod ball;
use ball::Ball;

mod game_pad_tracker;
use game_pad_tracker::GamePadTracker;

#[no_mangle]
unsafe fn setup() {

}

static mut FRAME: Frame = Frame::new(Position::new(-10., -10.));
static mut BALL: Ball = Ball::new(ORIGIN, Vector::new(0.0, 0.0), RADIUS);
static mut ACCELERATION: Vector = GRAVITY;
static mut INPUTS: GamePadTracker = GamePadTracker::new();

#[derive(Copy, Clone, Debug, Default)]
pub struct BallisticParameters {
    initial_position: Position,
    release_velocity: Vector,
    acceleration: Vector,
    time_in_flight: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum BallStatus {
    Reset(Position),
    Held(Position),
    Ballistic(BallisticParameters),
}

impl BallStatus {
    pub fn apply_transition(&mut self, transition: BallTransition) {
        match (&self, transition) {
            (Self::Reset(_), BallTransition::Grabbed(pos)) => *self = Self::Held(pos),
            (Self::Held(_), BallTransition::Released(params)) => *self = Self::Ballistic(params),
            (Self::Ballistic(BallisticParameters { initial_position, .. }), BallTransition::Reset) => *self = Self::Reset(*initial_position),
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BallTransition {
    Grabbed(Position),
    Released(BallisticParameters),
    Reset,
}

const GRAVITY: Vector = Vector::new(0., -10.0);

impl BallTransition {
    pub fn from_inputs(status: BallStatus, gpt: GamePadTracker, frame: Frame) -> Option<Self> {
        match status {
            BallStatus::Reset(_) if gpt.newly_clicked(MOUSE_LEFT) => Some(Self::Grabbed(frame.from_px_to_units(gpt.mouse_x as _, gpt.mouse_y as _))),
            BallStatus::Held(pos) if gpt.newly_released(MOUSE_LEFT) => Some(Self::Released(BallisticParameters {
                initial_position: ORIGIN,
                release_velocity: Vector::between(frame.from_px_to_units(gpt.mouse_x as _, gpt.mouse_y as _), pos),
                acceleration: GRAVITY,
                time_in_flight: 0.0
            })),
            BallStatus::Ballistic(params) if go_ballistic(
                params.initial_position,
                params.release_velocity,
                params.acceleration,
                params.time_in_flight,
            ).0.y <= params.initial_position.y => Some(Self::Reset),
            _ => None
        }
    }
}

static mut BALL_STATUS: BallStatus = BallStatus::Reset(ORIGIN);

const ORIGIN: Position = Position::new(0.0, 0.0);
const RADIUS: f32 = 5.0;

#[no_mangle]
unsafe fn update() {
    INPUTS.update(*GAMEPAD1, *MOUSE_BUTTONS, *MOUSE_X, *MOUSE_Y);

    if let Some(transition) = BallTransition::from_inputs(BALL_STATUS, INPUTS, FRAME) {
        BALL_STATUS.apply_transition(transition);
    }

    if let BallStatus::Ballistic(ref mut params) = BALL_STATUS {
        let (position, velocity) = go_ballistic(
            params.initial_position,
            params.release_velocity,
            params.acceleration,
            params.time_in_flight
        );
        BALL = Ball::new(position, velocity, RADIUS);
        params.time_in_flight += 1.0 / 60.0;
    }

    if let BallStatus::Reset(pos) = BALL_STATUS {
        BALL = Ball::new(pos, Default::default(), RADIUS);
    }
    
    // click and drag middle mouse button for this instead
    if INPUTS.pressed(BUTTON_RIGHT) {
        FRAME.mv(Vector::new(1.0, 0.0))
    }
    if INPUTS.pressed(BUTTON_LEFT) {
        FRAME.mv(Vector::new(-1.0, 0.0))
    }
    if INPUTS.pressed(BUTTON_DOWN) {
        FRAME.mv(Vector::new(0.0, -1.0))
    }
    if INPUTS.pressed(BUTTON_UP) {
        FRAME.mv(Vector::new(0.0, 1.0))
    }

    // button to lock/unlock screen from following ball

    FRAME.draw(BALL);
}

fn go_ballistic(position: Position, velocity: Vector, acceleration: Vector, time_seconds: f32) -> (Position, Vector) {
    let x = position.x + velocity.x * time_seconds + 0.5 * acceleration.x * time_seconds.powi(2);
    let y = position.y + velocity.y * time_seconds + 0.5 * acceleration.y * time_seconds.powi(2);

    let velocity = velocity + acceleration * time_seconds;

    (Position::new(x, y), velocity)
}

// #[derive(Copy, Clone, Debug)]
// enum BallState {
//     NotHeld,
//     Held {
//         position: Position,
//     },
//     Accelerating {
//         release_position: Position,
//         theta: f32,
//     },
//     Ballistic {
//         velocity: Vector,
//         position: Position,
//         net_force: Vector,
//     },
// }

// #[derive(Copy, Clone, Debug)]
// enum BallTransition {
//     Grab,
//     Release,
//     Freed,
//     Landed,
// }

// #[derive(Copy, Clone, Debug)]
// enum Force {
//     StaticFriction(f32),
//     KineticFriction(f32),
//     Field(Vector),
//     Simple(Vector),  // Normal or applied
// }

// #[derive(Copy, Clone)]
// enum Collision {
//     Elastic,  // both momentum and KE conserved
//     Inelastic,  // some KE is lost
//     TotalInelastic,  // stick together
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematics() {
        let pos = Position::new(0.0, 0.0);
        let vel = Vector::new(10., 10.);
        let acc = Vector::new(0., -1.);

        let (pos_final, vel_final) = go_ballistic(pos, vel, acc, 10.0);

        assert_eq!(pos_final.x, 100.0);
        assert_eq!(pos_final.y, 50.0);

        assert_eq!(vel_final.x, 10.0);
        assert_eq!(vel_final.y, 0.0);
    }
}