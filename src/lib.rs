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

mod game_pad_tracker;
use game_pad_tracker::GamePadTracker;

mod level_object;
use level_object::{GameState, GameMode};

mod constants;

#[no_mangle]
unsafe fn start() {
    *PALETTE = palettes::MOSSY;
    *DRAW_COLORS = 0x23;
}

mod palettes {
    const GRAY: u32 = 0xFFafb2a3;
    const DARK_GREEN: u32 = 0xFF2a4f1c;
    const ORANGE: u32 = 0xFFf49271;
    const GREEN: u32 = 0xFF587d3f;
    pub const MOSSY: [u32; 4] = [GREEN, DARK_GREEN, ORANGE, GRAY];
    pub const ICY: [u32; 4] = [0x03045e, 0x0096c7, 0x90e0ef, GRAY];
}

static mut GAME: GameState = GameState {
    gpt: GamePadTracker::new(),
    frame: Frame::new(Position::new(-80.0, -80.0)),
    mode: GameMode::TitleScreen,
};


// static mut OBJECTS: [LevelObject<Ball>; 3] = [
//     LevelObject::new(Projectile {
//             mass: RADIUS,
//             position: Position::new(40.0, 20.0),
//             velocity: Vector::new(0.0, 0.0),
//             object: Ball::new(RADIUS)
//         },
//         ProjectileStatus::Ballistic(Vector::new(0., 0.)),
//         ORIGIN,
//         GRAVITY
//     ),
//     LevelObject::new(Projectile {
//             mass: RADIUS,
//             position: Position::new(60.0, 0.0),
//             velocity: Vector::new(0.0, 0.0),
//             object: Ball::new(RADIUS)
//         },
//         ProjectileStatus::Reset,
//         ORIGIN,
//         GRAVITY
//     ),
//     LevelObject::new(Projectile {
//             mass: RADIUS,
//             position: Position::new(40.0, 40.0),
//             velocity: Vector::new(0.0, 0.0),
//             object: Ball::new(RADIUS)
//         },
//         ProjectileStatus::Ballistic(Vector::new(0., 0.)),
//         ORIGIN,
//         GRAVITY
//     )
// ];

#[no_mangle]
unsafe fn update() {
    GAME.update();
    GAME.draw();

    // for obj in OBJECTS.iter_mut() {
    //     obj.update_projectile_from_inputs(INPUTS, FRAME);
    //     obj.draw(FRAME);
    // }
    
    // click and drag middle mouse button for this instead
    // if INPUTS.pressed(BUTTON_RIGHT) {
    //     FRAME.mv(Vector::new(1.0, 0.0))
    // }
    // if INPUTS.pressed(BUTTON_LEFT) {
    //     FRAME.mv(Vector::new(-1.0, 0.0))
    // }
    // if INPUTS.pressed(BUTTON_DOWN) {
    //     FRAME.mv(Vector::new(0.0, -1.0))
    // }
    // if INPUTS.pressed(BUTTON_UP) {
    //     FRAME.mv(Vector::new(0.0, 1.0))
    // }

    // button to lock/unlock screen from following ball

    // collision physics

    // follow ball like mario's camera (after a certain line the frame moves)

    // FRAME.draw(BALL.object, BALL.position);
    // FRAME.draw(BALL_2.object, BALL_2.position);
}

// fn go_ballistic(position: Position, velocity: Vector, acceleration: Vector, time_seconds: f32) -> (Position, Vector) {
//     let x = position.x + velocity.x * time_seconds + 0.5 * acceleration.x * time_seconds.powi(2);
//     let y = position.y + velocity.y * time_seconds + 0.5 * acceleration.y * time_seconds.powi(2);

//     let velocity = velocity + acceleration * time_seconds;

//     (Position::new(x, y), velocity)
// }

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
// enum ProjectileTransition {
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
    // use super::*;

    
}