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
	pub fn empty() -> Self {
		World {
			level: [[Block::Air; 100]; 100],
		}
	}
}

