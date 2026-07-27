use crate::ASSETS;
use include_dir::File;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum Block {
	Air,
	Grass,
	Dirt,
	Stone,
}
pub struct World {
	pub level: HashMap<(i32, i32), [[Block; 100]; 100]>,
}
impl World {
	pub fn load_world() -> Self {
		let data: &str = ASSETS
			.get_file("level/data.txt")
			.unwrap_or_else(|| -> &File<'_> { panic!("Can't find text file data.txt") })
			.contents_utf8()
			.unwrap_or_else(|| -> &str { panic!("Can't convert from file to string") });

		let mut level: HashMap<(i32, i32), [[Block; 100]; 100]> = HashMap::new();

		for line_data in data.lines() {
			if line_data.is_empty() || &line_data[0..2] == "//" {
				continue;
			}

			let words: Vec<&str> = line_data.split_whitespace().collect();

			if let Ok(truncated_words) = words[..3].try_into() {
				let [x, y, path] = truncated_words;

				level.insert((x.parse().unwrap(), y.parse().unwrap()), {
					let chunk: &str = ASSETS
						.get_file(format!("level/{}", path))
						.unwrap_or_else(|| -> &File<'_> { panic!("Can't find text file {}", path) })
						.contents_utf8()
						.unwrap_or_else(|| -> &str { panic!("Can't convert {} from file to string", path) });

					let mut this_chunk: [[Block; 100]; 100] = [[Block::Air; 100]; 100];

					for (line_chunk, hor_slice) in chunk.lines().zip(this_chunk.iter_mut()) {
						for (r#char, block) in line_chunk.chars().zip(hor_slice.iter_mut()) {
							match r#char {
								's' => *block = Block::Stone,
								'g' => *block = Block::Grass,
								'd' => *block = Block::Dirt,
								'a' | ' ' => continue,
								_ => unreachable!("Invalid level encoding"),
							}
						}
					}

					this_chunk
				})
			} else {
				continue;
			};
		}

		Self { level }
	}

	pub fn get_block(&self, chunk_x: i32, chunk_y: i32, pos_x: usize, pos_y: usize) -> Block {
		self
			.level
			.get(&(chunk_x, chunk_y))
			.unwrap_or(&[[Block::Air; 100]; 100])[pos_y][pos_x]
	}
}
