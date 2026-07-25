use macroquad::prelude::*;

struct Texture {
	dirt: Texture2D,
	grass: Texture2D,
	stone: Texture2D,
}
impl Texture {
	async fn new() -> Self {
		Texture {
			dirt: load_texture("assets/textures/blocks/dirt.png")
				.await
				.expect("Can't find the dirt texture"),
			grass: load_texture("assets/textures/blocks/grass.png")
				.await
				.expect("Can't find the grass texture"),
			stone: load_texture("assets/textures/blocks/stone.png")
				.await
				.expect("Can't find the stone texture"),
		}
	}
}

#[macroquad::main("Find What is Behind the Gate")]
async fn main() {
	println!("Game Starts");

	let texture: Texture = Texture::new().await;
	println!("Texture loaded successfully");

	build_textures_atlas();
	println!("Builded atlas");

	loop {
		if is_key_pressed(KeyCode::Escape) {
			println!("ESC detected, program's ending");
			break;
		}
		next_frame().await;
	}

	println!("Game exited");
}
