use crate::entity::Entity;
use signed_vec::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Block {
	#[default]
	Air,
	Grass,
	Dirt,
	Stone,
	GateLocked,
	GateUnlocked,
	Cliff(u8),
}

pub struct World {
	pub level: SignedVec<SignedVec<Block>>,
	pub entities: Vec<Option<(i32, i32, Entity)>>,
}
impl World {
	pub fn change_block(&mut self, pos_x: i32, pos_y: i32, block: Block) {
		let mut binding: SignedVec<SignedVec<Block>> = self.level.clone();

		let row: &mut SignedVec<Block> = binding
			.unsure_read_from_index_mut(pos_y as isize)
			.expect("Try to assess out of the world using keys");

		row.write_from_index(pos_x as isize, block, Block::Air);

		self
			.level
			.write_from_index(pos_y as isize, row.clone(), SignedVec::new());
	}

	pub fn get_block(&self, pos_x: i32, pos_y: i32) -> Option<Block> {
		if let Some(hor_slice) = self.level.unsure_read_from_index(pos_y as isize) {
			hor_slice.unsure_read_from_index(pos_x as isize).copied()
		} else {
			None
		}
	}

	#[allow(dead_code)]
	pub fn is_block(&self, pos_x: i32, pos_y: i32, block: Block) -> bool {
		self.get_block(pos_x, pos_y) == Some(block)
	}

	pub fn is_it_one_of_these_blocks(&self, pos_x: i32, pos_y: i32, blocks: &[Block]) -> bool {
		self
			.get_block(pos_x, pos_y)
			.map(|block| -> bool { blocks.contains(&block) })
			.unwrap_or(false)
	}
}
