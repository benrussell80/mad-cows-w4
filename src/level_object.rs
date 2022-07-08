use std::{fmt::Debug};
use crate::constants::{TIME_STEP, BOUNCE_DAMPING_FACTOR, FRICTION_DAMPING_FACTOR, COLLISION_DAMPING_FACTOR, THRESHOLD_VELOCITY, GRAVITY};
// use crate::drawable::Drawable;
use crate::{vector::Vector};
// use crate::Player::Player;
// use crate::Player_status::PlayerStatus;
use crate::position::Position;
// use crate::Player_status::PlayerTransition;
use crate::game_pad_tracker::GamePadTracker;
use crate::frame::Frame;
// use crate::constants::TIME_STEP;
use crate::wasm4::*;
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Avatar {
    Normal,
    Longhorn,
    Dairy,
    Chocolate,
}

impl Avatar {
    pub fn get_hitbox(&self) -> Rect {
        // todo!();
        match self {
            _ => Rect::new(8.0, 8.0)
        }
    }

    pub const fn get_mass(&self) -> f32 {
        match self {
            _ => 10.0
        }
    }

    pub fn draw(&self, x: i32, y: i32) {

    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum EnemyAvatar {
    Farmer,
    Scarecrow,
}

impl EnemyAvatar {
    pub fn get_hitbox(&self) -> Rect {
        // todo!();
        match self {
            _ => Rect::new(8.0, 8.0)
        }
    }

    pub const fn get_mass(&self) -> f32 {
        match self {
            _ => 10.0
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LevelObjectKind {
    // Ground (implicit)
    Box,
    Player(Avatar),
    Log {
        is_vertical: bool,
        length: f32,
    },
    Enemy(EnemyAvatar),
}

impl LevelObjectKind {
    const BOX_SPRITE: &'static [u8] = &[
        0b11111111,
        0b11000011,
        0b10100101,
        0b10011001,
        0b10010001,
        0b10100101,
        0b11000011,
        0b11111111,
    ];
    pub fn get_hitbox(&self) -> Rect {
        match self {
            Self::Box => Rect { width: 8.0, height: 8.0 },
            Self::Player(avatar) => avatar.get_hitbox(),
            Self::Log { is_vertical: true, length } => Rect::new(3.0, *length),
            Self::Log { is_vertical: false, length } => Rect::new(*length, 3.0),
            Self::Enemy(e_avater) => e_avater.get_hitbox()
        }
    }

    pub fn draw(&self, x: u32, y: u32) {
        let Rect {width, height} = self.get_hitbox();
        match self {
            Self::Box => {
                // blit(Self::BOX_SPRITE, x as _, y as _,)
                unsafe { *DRAW_COLORS = 0x42; }
                blit(Self::BOX_SPRITE, x as _, y as _, width as _, height as _, BLIT_1BPP);
                // rect(x as _, y as _, width as _, height as _);
            },
            Self::Player(avatar) => avatar.draw(x as _, y as _),
            _ => {}
        }
    }

    pub const fn get_mass(&self) -> f32 {
        match self {
            Self::Box => 1.0,
            Self::Log { length, .. } => *length,
            Self::Player(avatar) => avatar.get_mass(),
            Self::Enemy(e_avatar) => e_avatar.get_mass(),
        }
    }

    // pub fn sprite(&self) -> &[u8] {
    //     match self {

    //     }
    // }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LevelObject {
    kind: LevelObjectKind,
    position: Position,
    velocity: Vector,
}

impl LevelObject {
    pub const fn new(kind: LevelObjectKind, position: Position, velocity: Vector) -> Self {
        Self { kind, position, velocity }
    }

    pub fn draw(&self, frame: Frame) {
        // get position based on frame data
        if let Some((x, y)) = frame.drawing_coords(self.position, self.kind.get_hitbox()) {
            self.kind.draw(x, y);
        }
    }

    pub fn step_physics(&mut self, physics: Physics) {
        self.velocity.x += physics.gravity.x * TIME_STEP;
        self.velocity.y += physics.gravity.y * TIME_STEP;
        self.position.x += self.velocity.x * TIME_STEP;
        self.position.y += self.velocity.y * TIME_STEP;

        // energy loss due to bouncing
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            if self.velocity.y < 0.0 {
                self.velocity.y *= -(1.0 - physics.bounce_damping_factor);
                if self.velocity.y < THRESHOLD_VELOCITY {
                    self.velocity.y = 0.0;
                }
            }
        }

        // energy loss due to friction
    }

    pub fn bounce(&mut self, bounce_damping_factor: f32) {
        self.velocity.y = -self.velocity.y * (1.0 - bounce_damping_factor);
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.kind.get_hitbox().intersects(self.position, other.kind.get_hitbox(), other.position)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn intersects(&self, self_position: Position, other: Self, other_position: Position) -> bool {
        self_position.x + self.width > other_position.x
        && other_position.x + other.width > self_position.x
        && self_position.y + self.height > other_position.y
        && other_position.y + other.height > self_position.y
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Physics {
    gravity: Vector,
    bounce_damping_factor: f32,
    friction_damping_factor: f32,
    collision_damping_factor: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            gravity: GRAVITY,
            bounce_damping_factor: BOUNCE_DAMPING_FACTOR,
            friction_damping_factor: FRICTION_DAMPING_FACTOR,
            collision_damping_factor: COLLISION_DAMPING_FACTOR,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    number: u8,
    objects: Vec<LevelObject>,
    physics: Physics,
}

impl LevelData {
    pub const fn new(number: u8, objects: Vec<LevelObject>, physics: Physics) -> Self {
        Self { number, objects, physics }
    }

    pub fn draw(&self, frame: Frame) {
        for obj in self.objects.iter() {
            obj.draw(frame);
        }
    }

    pub fn step_physics(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.step_physics(self.physics);
        }
    }

    pub fn load_levels() -> Vec<Vec<LevelObject>> {
        serde_json::from_str(include_str!("levels.json")).unwrap()
    }

    pub fn get_mut_player_object(&mut self) -> Option<&mut LevelObject> {
        self.objects.iter_mut().find(|obj| matches!(obj, LevelObject { kind: LevelObjectKind::Player(_), .. }))
    }

    pub fn get_player_object(&self) -> Option<&LevelObject> {
        self.objects.iter().find(|obj| matches!(obj, LevelObject { kind: LevelObjectKind::Player(_), .. }))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PlayerStatus {
    Reset,
    Held(Position),
    Ballistic(Vector),
}

impl PlayerStatus {
    pub fn apply_transition(&mut self, transition: PlayerTransition) {
        match (&self, transition) {
            (Self::Reset, PlayerTransition::Grabbed(pos)) => *self = Self::Held(pos),
            (Self::Held(held_pos), PlayerTransition::Released(release_pos)) => *self = Self::Ballistic(Vector::between(release_pos, *held_pos)),
            (Self::Ballistic(_), PlayerTransition::Reset) => *self = Self::Reset,
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PlayerTransition {
    Grabbed(Position),
    Released(Position),
    Reset,
}

#[derive(Debug, Clone)]
pub struct PlayingData {
    levels: Vec<LevelData>,
    current_level: usize,
    active_data: LevelData,
    player_status: PlayerStatus
}

impl PlayingData {
    pub fn get_player_transition(&self, gpt: GamePadTracker, frame: Frame) -> Option<PlayerTransition> {
        match self.player_status {
            PlayerStatus::Reset if gpt.newly_clicked(MOUSE_LEFT) => Some(PlayerTransition::Grabbed(frame.from_px_to_units(gpt.mouse_x as _, gpt.mouse_y as _))),
            PlayerStatus::Held(_) if gpt.newly_released(MOUSE_LEFT) => Some(PlayerTransition::Released(frame.from_px_to_units(gpt.mouse_x as _, gpt.mouse_y as _))),
            PlayerStatus::Ballistic(_) if gpt.newly_pressed(BUTTON_2) => Some(PlayerTransition::Reset),
            _ => None
        }
    }

    pub fn update_after_transition(&mut self) {
        if let Some(player) = self.active_data.get_mut_player_object() {
            match self.player_status {
                PlayerStatus::Ballistic(release_velocity) => player.velocity = release_velocity,
                PlayerStatus::Reset => self.reset_player(),
                _ => {},
            }
        }
    }

    pub fn reset_player(&mut self) {
        if let (Some(active_player), Some(og_player)) = (self.active_data.get_mut_player_object(), self.levels[self.current_level].get_player_object()) {
            *active_player = *og_player;
        }
    }

    pub fn restart_level(&mut self) {
        if self.current_level > self.levels.len() {
            self.active_data = self.levels[self.current_level].clone();
        }
    }

    pub fn apply_transition(&mut self, transition: PlayerTransition) {
        self.player_status.apply_transition(transition);
    }

    pub fn next_level(&mut self) {
        self.current_level += 1;
        self.restart_level();
    }

    pub fn update_collisions(&mut self) -> LevelStatus {
        // todo!();
        let objects = &mut self.active_data.objects;
        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                unsafe {
                    let obj1 = objects.get_unchecked(i);
                    let obj2 = objects.get_unchecked(j);

                    // let dot1 = Vector::between(obj1.position, obj2.position).normalize().dot(obj1.velocity.normalize()).abs();
                    // let b1 = 0.0 < dot1 && dot1 < 1.0;

                    // let dot2 = Vector::between(obj2.position, obj1.position).normalize().dot(obj2.velocity.normalize()).abs();
                    // let b2 = 0.0 < dot2 && dot2 < 1.0;

                    if obj1.collides_with(obj2) /* && (b1 || b2 ) */ {
                        // update obj1
                        let (m1, v1) = (obj1.kind.get_mass(), obj1.velocity);

                        // update obj2
                        let (m2, v2) = (obj2.kind.get_mass(), obj2.velocity);

                        let collision = Collision {
                            obj1: (m1, v1),
                            obj2: (m2, v2),
                            kind: CollisionKind::Damping(self.active_data.physics.bounce_damping_factor),
                        };

                        let (v1_f, v2_f) = collision.simulate();

                        objects.get_unchecked_mut(i).velocity = v1_f;
                        objects.get_unchecked_mut(j).velocity = v2_f;
                    }
                }
            }
        }
        LevelStatus::InProgress
    }

    pub fn step_physics(&mut self) {
        self.active_data.step_physics();
    }
}

#[derive(Clone, Debug)]
pub enum GameMode {
    TitleScreen,
    Playing(PlayingData),
    EndGame,
}

#[derive(Debug, Copy, Clone)]
pub enum LevelStatus {
    InProgress,
    Complete,
    Lost,
}

impl GameMode {
    pub fn draw(&self, frame: Frame) {
        match self {
            Self::TitleScreen => Self::draw_title(),
            Self::Playing(PlayingData { active_data, .. }) => Self::draw_level(frame, active_data),
            Self::EndGame => Self::draw_end_game(),
        }
    }

    fn draw_title() {
        text("Press X to play", 30, 50);
    }

    fn draw_level(frame: Frame, level_data: &LevelData) {
        // write level number in corner
        level_data.draw(frame);
    }

    fn draw_end_game() {
        text("Congrats! You won!", 30, 50);
    }

    pub fn update(&mut self, gpt: GamePadTracker, frame: Frame) {
        match self {
            Self::TitleScreen => self.update_title(gpt),
            Self::Playing(_) => self.update_playing(gpt, frame),
            Self::EndGame => self.update_end_game(gpt),
        }
    }

    fn update_title(&mut self, gpt: GamePadTracker) {
        if gpt.newly_pressed(BUTTON_1) {
            self.start_game();            
        }
    }

    fn update_end_game(&mut self, gpt: GamePadTracker) {
        if gpt.newly_pressed(BUTTON_1) {
            *self = Self::TitleScreen
        }
    }

    fn update_playing(&mut self, gpt: GamePadTracker, frame: Frame) {
        if let Self::Playing(playing_data) = self {
            // check for player transitions
            if let Some(transition) = playing_data.get_player_transition(gpt, frame) {
                // apply transition
                playing_data.apply_transition(transition);
                playing_data.update_after_transition();
            }
            // check for collisions on all objects
            let level_status = playing_data.update_collisions();
    
            // check for complete/win condition
            match level_status {
                LevelStatus::Complete => playing_data.next_level(),
                LevelStatus::Lost => playing_data.restart_level(),
                LevelStatus::InProgress => playing_data.step_physics(),
            }
        }
    }

    fn start_game(&mut self) {
        let level_objects = LevelData::load_levels();
        let physics: Physics = Default::default();
        let levels: Vec<_> = level_objects.into_iter().enumerate().map(|(i, objs)| {
            LevelData {
                number: i as _,
                objects: objs,
                physics: physics
            }
        }).collect();
        let current_level = 0;
        if current_level > levels.len() {
            *self = Self::EndGame
        } else {
            *self = Self::Playing(PlayingData {
                active_data: levels[current_level].clone(),
                levels,
                current_level,
                player_status: PlayerStatus::Reset
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub gpt: GamePadTracker,
    pub frame: Frame,
    pub mode: GameMode,
}

impl GameState {
    pub fn draw(&self) {
        self.mode.draw(self.frame);
    }

    pub fn update(&mut self) {
        unsafe {
            self.gpt.update(*GAMEPAD1, *MOUSE_BUTTONS, *MOUSE_X, *MOUSE_Y);
        }
        self.mode.update(self.gpt, self.frame);
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CollisionKind {
    Elastic,
    Damping(f32),
    PerfectlyInelastic,
}

#[derive(Debug, Copy, Clone)]
pub struct Collision {
    pub obj1: (f32, Vector),
    pub obj2: (f32, Vector),
    pub kind: CollisionKind,
}

impl Collision {
    pub fn simulate(&self) -> (Vector, Vector) {
        let (m1, v1) = self.obj1;
        let (m2, v2) = self.obj2;
        match self.kind {
            CollisionKind::PerfectlyInelastic => {
                let p1 = m1 * v1;
                let p2 = m2 * v2;

                let v_f = (p1 + p2) / (m1 + m2);

                (v_f, v_f)
            },
            kind => {
                let factor = if let CollisionKind::Damping(factor) = kind { factor } else { 0.0 };
                let p1 = m1 * v1;
                let p2 = m2 * v2;

                let mut v1_f = p2 / m1;
                let mut v2_f = p1 / m2;

                // only dampen x velocity (todo: change this to only dampen in any direction where they are colliding)
                v1_f.x *= 1.0 - factor;
                v2_f.x *= 1.0 - factor;

                (v1_f, v2_f)
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_levels() {
        LevelData::load_levels();
    }

    #[test]
    fn test_perfectly_inelastic_collision() {
        let col = Collision {
            obj1: (2.0, Vector::new(3.0, 0.0)),
            obj2: (1.0, Vector::new(0.0, 0.0)),
            kind: CollisionKind::PerfectlyInelastic,
        };

        let (v1, v2) = col.simulate();

        assert_eq!(v1, v2);
        assert_eq!(v1.magnitude(), 2.0);
        assert_eq!(v2.magnitude(), 2.0);
    }

    #[test]
    fn test_elastic_collision() {
        let v1 = Vector::new(3.0, 0.0);
        let v2 = Vector::new(0.0, 0.0);
        let m = 10.0;

        let col = Collision {
            obj1: (m, v1),
            obj2: (m, v2),
            kind: CollisionKind::Elastic,
        };

        let (v1_f, v2_f) = col.simulate();

        assert_eq!(v1_f, v2);
        assert_eq!(v2_f, v1);
    }

    #[test]
    fn test_elastic_collision_opposite_directions() {
        let v1 = Vector::new(-3.0, 0.0);
        let v2 = Vector::new(4.0, 0.0);
        let m = 10.0;

        let col = Collision {
            obj1: (m, v1),
            obj2: (m, v2),
            kind: CollisionKind::Elastic,
        };

        let (v1_f, v2_f) = col.simulate();

        assert_eq!(v1_f, v2);
        assert_eq!(v2_f, v1);
    }

    #[test]
    fn test_elastic_collision_opposite_directions_with_damping() {
        let v1 = Vector::new(-3.0, 0.0);
        let v2 = Vector::new(4.0, 0.0);
        let m = 10.0;
        let damping_factor = 0.05;

        let col = Collision {
            obj1: (m, v1),
            obj2: (m, v2),
            kind: CollisionKind::Damping(damping_factor),
        };

        let (v1_f, v2_f) = col.simulate();

        assert_eq!(v1_f, v2 * (1. - damping_factor));
        assert_eq!(v2_f, v1 * (1. - damping_factor));
    }
}