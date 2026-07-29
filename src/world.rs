use std::str::SplitWhitespace;

use crate::ASSETS;
use include_dir::File;
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
}
pub struct World {
	pub level: SignedVec<SignedVec<Block>>,
}
impl World {
	pub fn load_world() -> Self {
		let data: &str = ASSETS
			.get_file("level/data.txt")
			.unwrap_or_else(|| -> &File<'_> { panic!("Can't find text file data.txt") })
			.contents_utf8()
			.unwrap_or_else(|| -> &str { panic!("Can't convert from file to string") });

		let mut level: SignedVec<SignedVec<Block>> = SignedVec::new();

		for line in data.lines() {
			if line.is_empty() || line[0..2] == *"//" {
				continue;
			}

			match line.chars().nth(0).unwrap_or_else(|| -> char {
				unreachable!(
					"Can't have an string that is smaller than 1 char if 0 char is taken cared of in Ln 31"
				)
			}) {
				'b' => Self::load_blocks(&mut level, line),
				'e' => todo!(),
				_ => unreachable!("Invalid data.txt format"),
			}
		}

		Self { level }
	}

	fn load_blocks(level: &mut SignedVec<SignedVec<Block>>, line: &str) {
		let mut chunks: SplitWhitespace<'_> = line.split_whitespace();
		let _ = chunks.next();
		let starting_x: isize = chunks
			.next()
			.unwrap_or_else(|| -> &str { unreachable!("Invalide encoding with the var x") })
			.parse()
			.expect("Invalid base for x");
		let starting_y: isize = chunks
			.next()
			.unwrap_or_else(|| -> &str { unreachable!("Invalide encoding with the var y") })
			.parse()
			.expect("Invalid base for y");
		let file: &str = chunks
			.next()
			.unwrap_or_else(|| unreachable!("Invalide encoding with the var x"));
		let data: &str = ASSETS
			.get_file(format!("level/{}.txt", file))
			.unwrap_or_else(|| -> &File<'_> { panic!("Can't find the text file {}.txt", file) })
			.contents_utf8()
			.unwrap_or_else(|| -> &str { panic!("Can't convert from file to string") });

		for (row_offset, line_data) in data.lines().enumerate() {
			let y: isize = starting_y + row_offset as isize;

			let new: SignedVec<Block> = SignedVec::new();

			let mut val_y: SignedVec<Block> = level.unsure_read_from_index(y).unwrap_or(&new).clone();

			for (col_offset, r#char) in line_data.chars().enumerate() {
				let x: isize = starting_x + col_offset as isize;
				let block: Block = match r#char {
					'g' => Block::Grass,
					'd' => Block::Dirt,
					's' => Block::Stone,
					'l' => Block::GateLocked,
					'u' => Block::GateUnlocked,
					'a' | ' ' => Block::Air,
					_ => unreachable!("Invalid level encoding in {}.txt", file),
				};

				val_y.write_from_index(x, block, Block::Air);
			}

			level.write_from_index(y, val_y.clone(), SignedVec::new());
		}
	}

	fn get_block(&self, pos_x: isize, pos_y: isize) -> Option<Block> {
		if let Some(hor_slice) = self.level.unsure_read_from_index(pos_y) {
			hor_slice.unsure_read_from_index(pos_x).copied()
		} else {
			None
		}
	}

	#[allow(dead_code)]
	pub fn is_block(&self, pos_x: isize, pos_y: isize, block: Block) -> bool {
		self.get_block(pos_x, pos_y) == Some(block)
	}

	pub fn is_it_one_of_these_blocks(&self, pos_x: isize, pos_y: isize, blocks: &[Block]) -> bool {
		blocks.contains(&{
			let pos_block = self.get_block(pos_x, pos_y);
			if pos_block.is_none() {
				return false;
			}
			pos_block.unwrap_or_else(|| unreachable!("Impossible to reach, `None` cased covered"))
		})
	}
}
