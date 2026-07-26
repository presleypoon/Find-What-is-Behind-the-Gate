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
	
	draw_texture_ex(
		&texture.player,
		player.pos.x,
		player.pos.y,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(16.0, 16.0)),
			source: Some(Rect {
				x: (9.0 - player.sprite as f32).abs() * 16.0,
				y: 0.0,
				w: 16.0,
				h: 16.0,
			}),
			rotation: 0.0,
			flip_x: false,
			flip_y: false,
			pivot: None,
		},
	);
}
