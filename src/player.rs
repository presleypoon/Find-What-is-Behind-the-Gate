use crate::world::*;
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
			pos: vec2(25.0, 19.0),
			sprite_sub_tick: 0,
			sprite: 0,
			dir: Dir::N,
		}
	}

	pub fn update(&mut self, world: &World) {
		if !is_any_key_down() {
			return;
		}

		let key_in: Vec2 = Self::key_in();

		self.change_pos(world, key_in);

		self.sprite_sub_tick += 1;

		while self.sprite_sub_tick > 10 {
			self.sprite_sub_tick -= 10;
			self.sprite += 1;
			self.sprite %= 15;
		}

		self.dir = match (key_in.x, key_in.y) {
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

	fn change_pos(&mut self, world: &World, key_in: Vec2) {
		const VEL: f32 = 0.8;

		self.change_x(world, key_in.x * VEL);
		self.change_y(world, -key_in.y * VEL);
	}

	fn change_x(&mut self, world: &World, x: f32) {
		let sign_x: f32 = x.signum();

		for _ in 0..((x * 50.0) as i32).unsigned_abs() {
			self.pos.x += sign_x / 80.0;
			let blocks: &Vec<Block> = &vec![Block::Stone, Block::GateLocked];

			if world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
				blocks,
			) {
				self.pos.x -= sign_x / 80.0;
				break;
			}

			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
			) && matches!(block, Block::Cliff(4..16))
			{
				if sign_x == -1.0 {
					if (self.pos.x + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.x -= sign_x / 80.0;
						break;
					}
				} else if sign_x == 1.0 && (self.pos.x + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.x -= sign_x / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
			) && matches!(block, Block::Cliff(4..16))
			{
				if sign_x == -1.0 {
					if (self.pos.x + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.x -= sign_x / 80.0;
						break;
					}
				} else if sign_x == 1.0 && (self.pos.x + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.x -= sign_x / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
			) && matches!(block, Block::Cliff(4..16))
			{
				if sign_x == -1.0 {
					if (self.pos.x + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.x -= sign_x / 80.0;
						break;
					}
				} else if sign_x == 1.0 && (self.pos.x + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.x -= sign_x / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
			) && matches!(block, Block::Cliff(4..16))
			{
				if sign_x == -1.0 {
					if (self.pos.x + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.x -= sign_x / 80.0;
						break;
					}
				} else if sign_x == 1.0 && (self.pos.x + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.x -= sign_x / 80.0;
					break;
				}
			}
		}
	}

	fn change_y(&mut self, world: &World, y: f32) {
		let sign_y: f32 = y.signum();

		for _ in 0..((y * 50.0) as i32).unsigned_abs() {
			self.pos.y += sign_y / 80.0;
			let blocks: &Vec<Block> = &vec![Block::Stone, Block::GateLocked];

			if world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
				blocks,
			) || world.is_it_one_of_these_blocks(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
				blocks,
			) {
				self.pos.y -= sign_y / 80.0;
				break;
			}

			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
			) && matches!(block, Block::Cliff(1 | 3 | 4..=7 | 9 | 11..=15))
			{
				if sign_y == -1.0 {
					if (self.pos.y + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.y -= sign_y / 80.0;
						break;
					}
				} else if sign_y == 1.0 && (self.pos.y + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.y -= sign_y / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 16.0).floor() as i32,
			) && matches!(block, Block::Cliff(1 | 3 | 4..=7 | 9 | 11..=15))
			{
				if sign_y == -1.0 {
					if (self.pos.y + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.y -= sign_y / 80.0;
						break;
					}
				} else if sign_y == 1.0 && (self.pos.y + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.y -= sign_y / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 16.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
			) && matches!(block, Block::Cliff(1 | 3 | 4..=7 | 9 | 11..=15))
			{
				if sign_y == -1.0 {
					if (self.pos.y + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.y -= sign_y / 80.0;
						break;
					}
				} else if sign_y == 1.0 && (self.pos.y + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.y -= sign_y / 80.0;
					break;
				}
			}
			if let Some(block) = world.get_block(
				(self.pos.x + 3.0 / 4.0).floor() as i32,
				(self.pos.y + 3.0 / 4.0).floor() as i32,
			) && matches!(block, Block::Cliff(1 | 3 | 4..=7 | 9 | 11..=15))
			{
				if sign_y == -1.0 {
					if (self.pos.y + 3.0 / 16.0).fract() <= 1.0 / 8.0 {
						self.pos.y -= sign_y / 80.0;
						break;
					}
				} else if sign_y == 1.0 && (self.pos.y + 3.0 / 4.0).fract() >= 7.0 / 8.0 {
					self.pos.y -= sign_y / 80.0;
					break;
				}
			}
		}
	}

	fn key_in() -> Vec2 {
		let mut ret: Vec2 = Vec2::ZERO;

		if is_key_down(KeyCode::W) {
			ret += vec2(0.0, 1.0);
		}
		if is_key_down(KeyCode::A) {
			ret += vec2(-1.0, 0.0);
		}
		if is_key_down(KeyCode::S) {
			ret += vec2(0.0, -1.0);
		}
		if is_key_down(KeyCode::D) {
			ret += vec2(1.0, 0.0);
		}

		ret
	}
}

