use crate::{player::*, texture::*, world::*};
use macroquad::prelude::*;

pub fn render(player: &Player, world: &World, texture: &Texture) {
	clear_background(BLACK);
	for ((x, y), chunk) in &world.level {
		for (i, hor_slice) in chunk.iter().enumerate() {
			if i as f32 * 16.0 < player.pos.y - 16.0 || i as f32 * 16.0 > player.pos.y + 800.0 {
				continue;
			}
			for (j, block) in hor_slice.iter().enumerate() {
				let block_x = (x * 1600) as f32 + j as f32 * 16.0;
				let block_y = (y * 1600) as f32 + i as f32 * 16.0;

				if block_x < player.pos.x - 16.0
					|| block_x > player.pos.x + 800.0
					|| block_y < player.pos.y - 16.0
					|| block_y > player.pos.y + 600.0
				{
					continue;
				}

				draw_texture(
					match block {
						Block::Air => continue,
						Block::Grass => &texture.grass,
						Block::Dirt => &texture.dirt,
						Block::Stone => &texture.stone,
					},
					(block_x - player.pos.x).round(),
					(block_y - player.pos.y).round(),
					WHITE,
				);
			}
		}
	}

	draw_texture(
		&texture.player[(8 - player.sprite as isize).unsigned_abs()],
		400.0,
		300.0,
		WHITE,
	);
}
