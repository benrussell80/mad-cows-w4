use std::{fmt::Debug, thread::current};
// use crate::drawable::Drawable;
use crate::vector::Vector;
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
            _ => Rect::new(10.0, 10.0)
        }
    }

    pub const fn get_mass(&self) -> f32 {
        match self {
            _ => 10.0
        }
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
            _ => Rect::new(10.0, 10.0)
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
    pub fn get_hitbox(&self) -> Rect {
        match self {
            Self::Box => Rect { width: 10.0, height: 10.0 },
            Self::Player(avatar) => avatar.get_hitbox(),
            Self::Log { is_vertical: true, length } => Rect::new(3.0, *length),
            Self::Log { is_vertical: false, length } => Rect::new(*length, 3.0),
            Self::Enemy(e_avater) => e_avater.get_hitbox()
        }
    }

    pub fn draw(&self, x: u32, y: u32) {
        let Rect {width, height} = self.get_hitbox();
        match self {
            _ => {
                unsafe { *DRAW_COLORS = 0x11; }
                rect(x as _, y as _, width as _, height as _);
            },

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
        self.velocity.x += physics.gravity.x;
        self.velocity.y += physics.gravity.y;
        self.position.x += self.velocity.x;
        self.position.y = (self.position.y + self.velocity.y).max(0.0);
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
}

impl Default for Physics {
    fn default() -> Self {
        Self { gravity: Vector { x: 0.0, y: -10.0 } }
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

    pub fn update(&mut self, gpt: GamePadTracker) {
        // check for collisions, player movements, etc.

        // press x to change avatar
        for obj in self.objects.iter_mut() {
            obj.step_physics(self.physics);
        }
    }

    pub fn load_levels() -> Vec<LevelData> {
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


#[derive(Clone, Debug)]
pub enum GameMode {
    TitleScreen,
    Playing {
        levels: Vec<LevelData>,
        current_level: usize,
        active_data: LevelData,
        player_status: PlayerStatus
    },
    EndGame,
}

impl GameMode {
    pub fn draw(&self, frame: Frame) {
        match self {
            Self::TitleScreen => Self::draw_title(),
            Self::Playing { active_data, .. } => Self::draw_level(frame, active_data),
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

    pub fn update(&mut self, gpt: GamePadTracker) {
        match self {
            Self::TitleScreen => {
                if gpt.newly_pressed(BUTTON_1) {
                    let levels = LevelData::load_levels();
                    *self = Self::Playing { active_data: levels[0].clone(), levels, current_level: 0, player_status: PlayerStatus::Reset }
                }
            },
            Self::Playing { levels, active_data, current_level, player_status } => {
                if gpt.newly_pressed(BUTTON_1) {
                    // restart
                    *active_data = levels[*current_level].clone();
                }

                if gpt.newly_pressed(BUTTON_1) {
                    // reset
                    if let (Some(active_player), Some(og_player)) = (active_data.get_mut_player_object(), levels[*current_level].get_player_object()) {
                        *active_player = *og_player;
                    }
                }

                // check for player inputs

                // check for collisions

                // check for complete/win condition
            },
            Self::EndGame => {
                if gpt.newly_pressed(BUTTON_1) {
                    *self = Self::TitleScreen
                }
            }
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
        self.mode.update(self.gpt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_levels() {
        LevelData::load_levels();
    }
}