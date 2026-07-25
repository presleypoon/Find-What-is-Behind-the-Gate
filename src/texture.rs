use macroquad::prelude::*;

pub struct Texture {
	pub dirt: Texture2D,
	pub grass: Texture2D,
	pub stone: Texture2D,
}
impl Texture {
	pub async fn new() -> Self {
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

		Self { dirt, grass, stone }
	}
}
