use macroquad::prelude::*;

pub struct Texture {
	pub dirt: Texture2D,
	pub grass: Texture2D,
	pub stone: Texture2D,
	pub player: Texture2D,
}
impl Texture {
	pub async fn new() -> Self {
		let dirt = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/dirt.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the PNG dirt"),
		);
		let grass = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/grass.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the PNG grass"),
		);
		let stone = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/stone.png"),
				Some(ImageFormat::Png),
			)
			.expect("Can't find the PNG stone"),
		);
		let player = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/entity/player.png"), Some(ImageFormat::Png),
			).expect("Can't find the PNG player")
		);

		Self { dirt, grass, stone, player }
	}
}
