use macroquad::prelude::*;

pub struct Player {
	pub pos: Vec2,
}
impl Player {
	pub fn new() -> Self {
		Self { pos: Vec2::ZERO }
	}
}
