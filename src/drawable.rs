use crate::position::Position;


pub trait Drawable {
    fn get_position(&self) -> Position;
    fn get_height(&self) -> f32;
    fn get_width(&self) -> f32;
    fn draw(&self, center: (i32, i32), scale: f32);
}