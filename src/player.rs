use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum Dir {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

pub struct Player {
	pub pos: Vec2,
	sprite_sub_tick: u8,
	pub sprite: u8,
	pub dir: Dir,
}
impl Player {
	pub fn new() -> Self {
		Self {
			pos: Vec2::ZERO,
			sprite_sub_tick: 0,
			sprite: 0,
			dir: Dir::N,
		}
	}

	pub fn r#move(&mut self) {
		if !is_any_key_down() {
			return;
		}

		const VEL: f32 = 0.8;

		let mut vel_after_key_press: Vec2 = Vec2::ZERO;

		if is_key_down(KeyCode::W) {
			self.pos += vec2(0.0, -VEL);
			vel_after_key_press += vec2(0.0, 1.0);
		}
		if is_key_down(KeyCode::A) {
			self.pos += vec2(-VEL, 0.0);
			vel_after_key_press += vec2(-1.0, 0.0);
		}
		if is_key_down(KeyCode::S) {
			self.pos += vec2(0.0, VEL);
			vel_after_key_press += vec2(0.0, -1.0);
		}
		if is_key_down(KeyCode::D) {
			self.pos += vec2(VEL, 0.0);
			vel_after_key_press += vec2(1.0, 0.0);
		}

		self.sprite_sub_tick += 1;

		while self.sprite_sub_tick > 10 {
			self.sprite_sub_tick -= 10;
			self.sprite += 1;

			self.sprite %= 15;
		}

		self.dir = match (vel_after_key_press.x, vel_after_key_press.y) {
			(1.0, 0.0) => Dir::N,
			(1.0, 1.0) => Dir::NE,
			(0.0, 1.0) => Dir::E,
			(-1.0, 1.0) => Dir::SE,
			(-1.0, 0.0) => Dir::S,
			(-1.0, -1.0) => Dir::SW,
			(0.0, -1.0) => Dir::W,
			(1.0, -1.0) => Dir::NW,
			_ => self.dir,
		}
	}
}
