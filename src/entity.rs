use crate::{player::*, world::*};

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum Entity {
	Key(Vec<(i32, i32)>),
}

impl World {
	pub fn entities_update(&mut self, player: &Player) {
		for i in 0..self.entities.iter().len() {
			if let Some((x, y, entity)) = &self.entities[i] {
				match entity {
					Entity::Key(gate) => self.key_update((*x, *y), gate.to_vec(), player, i),
				}
			}
		}
	}

	fn key_update(&mut self, pos: (i32, i32), gate: Vec<(i32, i32)>, player: &Player, i: usize) {
		let tl: (i32, i32) = (
			(player.pos.x + 3.0 / 16.0) as i32,
			(player.pos.y + 3.0 / 16.0) as i32,
		);
		let tr: (i32, i32) = (
			(player.pos.x + 3.0 / 16.0) as i32,
			(player.pos.y + 12.0 / 16.0) as i32,
		);
		let bl: (i32, i32) = (
			(player.pos.x + 12.0 / 16.0) as i32,
			(player.pos.y + 3.0 / 16.0) as i32,
		);
		let br: (i32, i32) = (
			(player.pos.x + 12.0 / 16.0) as i32,
			(player.pos.y + 12.0 / 16.0) as i32,
		);

		if pos != tl && pos != tr && pos != bl && pos != br {
			return;
		}

		for (gx, gy) in gate {
			self.change_block(gx, gy, Block::GateUnlocked);
		}

		self.entities[i] = None;
	}
}
