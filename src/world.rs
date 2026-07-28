use crate::ASSETS;
use include_dir::File;
use signed_vec::*;

#[derive(Clone, Copy, PartialEq, Default)]
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
		let mut chunks: std::str::SplitWhitespace<'_> = line.split_whitespace();

		let _ = chunks.next();

		let starting_x: &str = chunks
			.next()
			.unwrap_or_else(|| -> &str { unreachable!("Invalid encoding with var x") });
		let starting_y: &str = chunks
			.next()
			.unwrap_or_else(|| -> &str { unreachable!("Invalid encoding with var y") });
		let file: &str = chunks
			.next()
			.unwrap_or_else(|| -> &str { unreachable!("Invalid encoding with var file") });

		let data: &str = ASSETS
			.get_file(format!("level/{}.txt", file))
			.unwrap_or_else(|| -> &File<'_> { unreachable!("Can't find text file {}.txt", file) })
			.contents_utf8()
			.unwrap_or_else(|| -> &str { unreachable!("Can't convert from file to string") });

		for (y, line_data) in (starting_y.parse::<isize>().unwrap_or_else(|e: std::num::ParseIntError| -> isize {
				unreachable!("Invalid base for y with {} error", e);
			})..).zip(data.lines()) {
			let mut val_y: SignedVec<Block> = SignedVec::new();

			for (x, r#char) in (starting_x.parse::<isize>().unwrap_or_else(
				|e: std::num::ParseIntError| -> isize {
					unreachable!("Invalid base for x with {} error", e);
				},
			)..).zip(line_data.chars()) {
				val_y.write_from_index(
					x,
					match r#char {
						'g' => Block::Grass,
						'd' => Block::Dirt,
						's' => Block::Stone,
						'l' => Block::GateLocked,
						'u' => Block::GateUnlocked,
						'a' | ' ' => Block::Air,
						_ => unreachable!("Invalid letter encoding in {}, at {}:{}", file, x, y),
					},
					Block::Air,
				);
			}

			level.write_from_index(y, val_y, SignedVec::new());
		}
	}

	fn get_block(&self, pos_x: isize, pos_y: isize) -> Block {
		*self.level.read_from_index(pos_y).read_from_index(pos_x)
	}

	#[allow(dead_code)]
	pub fn is_block(
		&self,
		pos_x: isize,
		pos_y: isize,
		block: Block,
	) -> bool {
		self.get_block(pos_x, pos_y) == block
	}

	pub fn is_it_one_of_these_blocks(
		&self,
		pos_x: isize,
		pos_y: isize,
		blocks: &[Block],
	) -> bool {
		blocks.contains(&self.get_block(pos_x, pos_y))
	}
}
