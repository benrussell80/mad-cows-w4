use crate::wasm4::*;
use crate::position::Position;
use crate::drawable::Drawable;
use crate::vector::Vector;


#[derive(Copy, Clone, Debug)]
pub struct Frame {
    anchor_point: Position,
}


impl Frame {
    pub const fn new(anchor_point: Position) -> Self {
        Self {
            anchor_point,
        }
    }

    pub const fn get_position(&self) -> Position {
        self.anchor_point
    }

    pub fn mv(&mut self, offset: Vector) {
        self.anchor_point.x += offset.x;
        self.anchor_point.y += offset.y;
    }

    pub fn from_px_to_units(&self, x: i32, y: i32) -> Position {
        let mut pos: Position = Default::default();
        let frame_width = SCREEN_SIZE as f32;
        let distance_from_anchor_x = (x as f32) / SCREEN_SIZE as f32 * frame_width;
        pos.x = distance_from_anchor_x + self.anchor_point.x;

        let frame_height = SCREEN_SIZE as f32;
        let distance_from_anchor_y = ((SCREEN_SIZE as i32 - y) as f32) / SCREEN_SIZE as f32 * frame_height;
        pos.y = distance_from_anchor_y + self.anchor_point.y;
        pos
    }

    pub fn from_units_to_px(&self, pos: Position) -> (i32, i32) {
        let distance_from_anchor_x = pos.x - self.anchor_point.x;
        let frame_width = SCREEN_SIZE as f32;
        let x = (SCREEN_SIZE as f32 * (distance_from_anchor_x / frame_width)) as i32;

        let distance_from_anchor_y = pos.y - self.anchor_point.y;
        let frame_height = SCREEN_SIZE as f32;
        let y = SCREEN_SIZE as i32 - (SCREEN_SIZE as f32 * (distance_from_anchor_y / frame_height)) as i32;
        (x, y)
    }

    pub fn draw(&self, object: impl Drawable) {
        let x0 = self.anchor_point.x;
        let x1 = x0 + SCREEN_SIZE as f32;

        let y0 = self.anchor_point.y;
        let y1 = y0 + SCREEN_SIZE as f32;

        let obj_pos = object.get_position();
        if obj_pos.contained_within(
            Position::new(x0, y0), Position::new(x1, y1)
        ) {
            // calculate where it should be drawn
            let (x, y) = self.from_units_to_px(obj_pos);
            object.draw((x, y), 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let pos = Position::new(0., 0.);

        let frame = Frame::new(pos);

        let (x, y) = frame.from_units_to_px(pos);
        let pos2 = frame.from_px_to_units(x, y);

        assert_eq!(pos, pos2);
    }
}