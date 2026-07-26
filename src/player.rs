use macroquad::prelude::*;

pub struct Player {
	pub pos: Vec2,
}
impl Player {
	pub fn new() -> Self {
		Self { pos: Vec2::ZERO }
	}

	pub fn r#move(&mut self) {
		if !is_any_key_down() {
			return;
		}

		if is_key_down(KeyCode::W) {
			self.pos += vec2(0.0, -1.0);
		}
		if is_key_down(KeyCode::A) {
			self.pos += vec2(-1.0, 0.0);
		}
		if is_key_down(KeyCode::S) {
			self.pos += vec2(0.0, 1.0);
		}
		if is_key_down(KeyCode::D) {
			self.pos += vec2(1.0, 0.0);
		}
	}
}
