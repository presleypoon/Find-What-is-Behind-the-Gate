use macroquad::prelude::*;

pub struct Player {
	pub pos: Vec2,
}
impl Player {
	pub fn new() -> Self {
		Player { pos: Vec2::ZERO }
	}
}
