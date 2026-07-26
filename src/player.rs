use macroquad::prelude::*;

pub struct Player {
	pub pos: Vec2,
	pub sprite: u8,
}
impl Player {
	pub fn new() -> Self {
		Self {
			pos: Vec2::ZERO,
			sprite: 0,
		}
	}

	pub fn r#move(&mut self) {
		if !is_any_key_down() {
			return;
		}

		const VEL: f32 = 0.8;

		if is_key_down(KeyCode::W) {
			self.pos += vec2(0.0, -VEL);
		}
		if is_key_down(KeyCode::A) {
			self.pos += vec2(-VEL, 0.0);
		}
		if is_key_down(KeyCode::S) {
			self.pos += vec2(0.0, VEL);
		}
		if is_key_down(KeyCode::D) {
			self.pos += vec2(VEL, 0.0);
		}
	}
}
