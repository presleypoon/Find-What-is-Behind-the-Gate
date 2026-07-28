use crate::{player::*, texture::*, world::*};
use macroquad::prelude::*;

pub fn render(player: &Player, world: &World, texture: &Texture) {
	clear_background(BLACK);
	let y_range: (usize, usize) = world.level.range();
	for y in (-(y_range.0 as isize))..(y_range.1 as isize) {
		let block_y: f32 = y as f32 * 16.0 + 300.0;
		if block_y < player.pos.y * 16.0 - 16.0 || block_y > player.pos.y * 16.0 + 600.0 {
			continue;
		}
		let hor_slice: &signed_vec::SignedVec<Block> = world.level.read_from_index(y);
		let x_range: (usize, usize) = hor_slice.range();
		for x in (-(x_range.0 as isize))..(x_range.1 as isize) {
			let block_x: f32 = x as f32 * 16.0 + 400.0;

			if block_x < player.pos.x * 16.0 - 16.0 || block_x > player.pos.x * 16.0 + 800.0 {
				continue;
			}

			let block = hor_slice.read_from_index(x);

			draw_texture(
				match block {
					Block::Air => continue,
					Block::Grass => &texture.grass,
					Block::Dirt => &texture.dirt,
					Block::Stone => &texture.stone,
					Block::GateLocked => &texture.gate_locked,
					Block::GateUnlocked => &texture.gate_unlocked,
				},
				(block_x - player.pos.x * 16.0).round(),
				(block_y - player.pos.y * 16.0).round(),
				WHITE,
			);
		}
	}

	draw_texture_ex(
		&texture.player[(8 - player.sprite as isize).unsigned_abs()],
		400.0,
		300.0,
		WHITE,
		DrawTextureParams {
			rotation: match player.dir {
				Dir::N => 90.0_f32,
				Dir::NE => 45.0,
				Dir::E => 0.0,
				Dir::SE => 315.0,
				Dir::S => 270.0,
				Dir::SW => 225.0,
				Dir::W => 180.0,
				Dir::NW => 135.0,
			}
			.to_radians(),
			..Default::default()
		},
	);
}
