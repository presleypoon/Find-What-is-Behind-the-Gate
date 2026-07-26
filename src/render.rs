use crate::{player::*, texture::*, world::*};
use macroquad::prelude::*;

pub fn render(player: &Player, world: &World, texture: &Texture) {
	clear_background(BLACK);

	for (i, hor_slice) in world.level.iter().enumerate() {
		if i as f32 * 16.0 < player.pos.y - 16.0 || i as f32 * 16.0 > player.pos.y + 800.0 {
			continue;
		}
		for (j, block) in hor_slice.iter().enumerate() {
			if j as f32 * 16.0 < player.pos.x - 16.0 || j as f32 * 16.0 > player.pos.x + 800.0 {
				continue;
			}

			draw_texture(
				match block {
					Block::Air => continue,
					Block::Grass => &texture.grass,
					Block::Dirt => &texture.dirt,
					Block::Stone => &texture.stone,
				},
				j as f32 * 16.0 - player.pos.x,
				i as f32 * 16.0 - player.pos.y,
				WHITE,
			);
		}
	}

	draw_texture(
		&texture.player[(8 - player.sprite as isize).unsigned_abs()],
		400.0,
		400.0,
		WHITE,
	);
}
