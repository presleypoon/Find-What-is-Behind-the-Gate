#[derive(Clone, Copy)]
pub enum Block {
	Air,
	Grass,
	Dirt,
	Stone,
}
pub struct World {
	pub level: [[Block; 100]; 100],
}
impl World {
	pub fn load_world(world: String) -> Self {
		let mut level: [[Block; 100]; 100] = [[Block::Air; 100]; 100];

		for (line, hor_slice) in world.lines().zip(level.iter_mut()) {
			for (r#char, block) in line.chars().zip(hor_slice.iter_mut()) {
				*block = match r#char {
					's' => Block::Stone,
					'd' => Block::Dirt,
					'g' => Block::Grass,
					'a' => Block::Air,
					_ => unreachable!("Invalid level encoding"),
				}
			}
		}

		Self { level }
	}
}
