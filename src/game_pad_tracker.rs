#[derive(Copy, Clone, Debug)]
pub struct GamePadTracker {
    current_gamepad: u8,
    prev_gamepad: u8,
    new_presses: u8,
    current_mouse: u8,
    prev_mouse: u8,
    new_clicks: u8,
    new_releases: u8,
    pub mouse_x: i16,
    pub mouse_y: i16,
}

impl GamePadTracker {
    pub const fn new() -> Self {
        Self {
            current_gamepad: 0,
            prev_gamepad: 0,
            new_presses: 0,
            prev_mouse: 0,
            current_mouse: 0,
            new_clicks: 0,
            new_releases: 0,
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    pub fn update(&mut self, current_gamepad: u8, current_mouse: u8, mouse_x: i16, mouse_y: i16) {
        self.prev_gamepad = self.current_gamepad;
        self.current_gamepad = current_gamepad;
        self.new_presses = self.current_gamepad & (self.current_gamepad ^ self.prev_gamepad);

        self.prev_mouse = self.current_mouse;
        self.current_mouse = current_mouse;
        self.new_clicks = self.current_mouse & (self.current_mouse ^ self.prev_mouse);
        self.new_releases = self.prev_mouse & (self.current_mouse ^ self.prev_mouse);

        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }

    pub const fn pressed(&self, key: u8) -> bool {
        self.current_gamepad & key != 0
    }

    pub const fn newly_pressed(&self, key: u8) -> bool {
        self.new_presses & key != 0
    }

    pub const fn clicked(&self, key: u8) -> bool {
        self.current_mouse & key != 0
    }

    pub const fn newly_clicked(&self, key: u8) -> bool {
        self.new_clicks & key != 0
    }

    pub const fn newly_released(&self, key: u8) -> bool {
        self.new_releases & key != 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm4::*;

    #[test]
    fn test_gamepad_tracker_newly_pressed() {
        let mut inputs = GamePadTracker::new();
        let gamepad = BUTTON_1 | BUTTON_2;
        inputs.update(gamepad, 0, 0, 0);
        assert!(inputs.newly_pressed(BUTTON_1));
        assert!(inputs.newly_pressed(BUTTON_2));
        assert!(!inputs.newly_pressed(BUTTON_UP));
        assert!(!inputs.newly_pressed(BUTTON_RIGHT));
        assert!(!inputs.newly_pressed(BUTTON_DOWN));
        assert!(!inputs.newly_pressed(BUTTON_LEFT));

        inputs.update(BUTTON_1 | BUTTON_UP, 0, 0, 0);
        assert!(!inputs.newly_pressed(BUTTON_1));
        assert!(!inputs.newly_pressed(BUTTON_2));
        assert!(!inputs.pressed(BUTTON_2));
        assert!(inputs.newly_pressed(BUTTON_UP));
    }

    #[test]
    fn test_gamepad_tracker_newly_released() {
        let mut inputs = GamePadTracker::new();
        inputs.update(0, MOUSE_LEFT | MOUSE_RIGHT, 0, 0);
        inputs.update(0, MOUSE_RIGHT, 0, 0);
        assert!(inputs.newly_released(MOUSE_LEFT));
        assert!(!inputs.newly_released(MOUSE_RIGHT));
    }
}