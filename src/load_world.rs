use std::str::SplitWhitespace;

use include_dir::File;
use signed_vec::*;

use crate::{entity::Entity, world::*, ASSETS};

impl World {
	pub fn load_world() -> Self {
		let data: &str = ASSETS
			.get_file("level/data.txt")
			.unwrap_or_else(|| -> &File<'_> { panic!("Can't find text file data.txt") })
			.contents_utf8()
			.unwrap_or_else(|| -> &str { panic!("Can't convert from file to string") });

		let mut level: SignedVec<SignedVec<Block>> = SignedVec::new();
		let mut entities: Vec<Option<(i32, i32, Entity)>> = Vec::new();

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
				'e' => Self::load_entities(&mut entities, line),
				_ => unreachable!("Invalid data.txt format"),
			}
		}

		Self { level, entities }
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
			.get_file(format!("level/{file}.txt"))
			.unwrap_or_else(|| -> &File<'_> { panic!("Can't find the text file {file}.txt") })
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
					'·' => Block::Cliff(0x0),
					'╵' => Block::Cliff(0x1),
					'╶' => Block::Cliff(0x2),
					'└' => Block::Cliff(0x3),
					'╷' => Block::Cliff(0x4),
					'│' => Block::Cliff(0x5),
					'┌' => Block::Cliff(0x6),
					'├' => Block::Cliff(0x7),
					'╴' => Block::Cliff(0x8),
					'┘' => Block::Cliff(0x9),
					'─' => Block::Cliff(0xA),
					'┴' => Block::Cliff(0xB),
					'┐' => Block::Cliff(0xC),
					'┤' => Block::Cliff(0xD),
					'┬' => Block::Cliff(0xE),
					'┼' => Block::Cliff(0xF),
					'a' | ' ' => Block::Air,
					_ => unreachable!("Invalid level encoding in {file}.txt"),
				};

				val_y.write_from_index(x, block, Block::Air);
			}

			level.write_from_index(y, val_y.clone(), SignedVec::new());
		}
	}

	fn load_entities(entity: &mut Vec<Option<(i32, i32, Entity)>>, line: &str) {
		let mut chunks: SplitWhitespace<'_> = line.split_whitespace();
		let _ = chunks.next();
		let x: i32 = chunks
			.next()
			.expect("No x coord provided for entity")
			.parse::<i32>()
			.expect("Can't decode entity's x");
		let y: i32 = chunks
			.next()
			.expect("No y coord provided for entity")
			.parse::<i32>()
			.expect("Can't decode entity's y");
		let name: &str = chunks.next().expect("No name provided for entity");

		match name {
			"key" => {
				let mut gate_x_y: Vec<(i32, i32)> = Vec::new();

				while let (Some(gate_x), Some(gate_y)) = (chunks.next(), chunks.next()) {
					let gate_x: i32 = gate_x.parse::<i32>().expect("gate number x is not an int");
					let gate_y: i32 = gate_y.parse::<i32>().expect("gate number y is not an int");
					gate_x_y.push((gate_x, gate_y));
				}

				entity.push(Some((x, y, Entity::Key(gate_x_y))));
			}
			_ => unreachable!("Invalid entity name"),
		}
	}
}
