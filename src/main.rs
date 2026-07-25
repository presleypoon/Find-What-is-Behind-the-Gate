use macroquad::prelude::*;

struct Player {
	pos: Vec2,
}
impl Player {
	fn new() -> Self {
		Player { pos: Vec2::ZERO }
	}
}

struct World {
	level: [[Block; 100]; 100],
}
impl World {
	fn empty() -> Self {
		World {
			level: [[Block::Air; 100]; 100],
		}
	}
}

#[derive(Clone, Copy)]
enum Block {
	Air,
	Grass,
	Dirt,
	Stone,
}

struct Texture {
	dirt: Texture2D,
	grass: Texture2D,
	stone: Texture2D,
}
impl Texture {
	async fn new() -> Self {
		let dirt = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/dirt.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the image dirt"),
		);
		let grass = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/grass.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the image grass"),
		);
		let stone = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/stone.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the image stone"),
		);

		Texture { dirt, grass, stone }
	}
}

#[macroquad::main("Find What is Behind the Gate")]
async fn main() {
	println!("Game Starts");

	let texture: Texture = Texture::new().await;
	println!("Texture init suc.");

	let player: Player = Player::new();
	println!("Player init suc.");

	let world: World = World::empty();
	println!("World init suc.");

	build_textures_atlas();
	println!("Builded atlas");

	loop {
		if is_key_pressed(KeyCode::Escape) {
			println!("ESC detected, program's ending");
			break;
		}

		render(&player, &world, &texture);

		next_frame().await;
	}

	println!("Game exited");
}

fn render(player: &Player, world: &World, texture: &Texture) {
	for hor_slice in world.level {}
}
