use macroquad::prelude::*;

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
			.unwrap(),
		);
		let grass = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/grass.png"),
				Some(ImageFormat::Png),
			)
			.unwrap(),
		);
		let stone = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/stone.png"),
				Some(ImageFormat::Png),
			)
			.unwrap(),
		);

		Texture { dirt, grass, stone }
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
