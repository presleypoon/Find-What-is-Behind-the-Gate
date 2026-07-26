use macroquad::prelude::*;

pub struct Texture {
	pub dirt: Texture2D,
	pub grass: Texture2D,
	pub stone: Texture2D,
	pub player: [Texture2D; 9],
}
impl Texture {
	pub async fn new() -> Self {
		let tnf: Image = Image::from_file_with_format(
			include_bytes!("../assets/textures/tnf.png"),
			Some(ImageFormat::Png),
		)
		.expect("Can't find image tnf");

		let dirt: Texture2D = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/dirt.png"),
				Some(ImageFormat::Png),
			)
			.unwrap_or(tnf.clone()),
		);
		let grass: Texture2D = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/grass.png"),
				Some(ImageFormat::Png),
			)
			.unwrap_or(tnf.clone()),
		);
		let stone: Texture2D = Texture2D::from_image(
			&Image::from_file_with_format(
				include_bytes!("../assets/textures/blocks/stone.png"),
				Some(ImageFormat::Png),
			)
			.unwrap_or(tnf.clone()),
		);
		let player: [Texture2D; 9] = [
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/0.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/1.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/2.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/3.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/4.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/5.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/6.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/7.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
			Texture2D::from_image(
				&Image::from_file_with_format(
					include_bytes!("../assets/textures/entity/player/8.png"),
					Some(ImageFormat::Png),
				)
				.unwrap_or(tnf.clone()),
			),
		];

		Self {
			dirt,
			grass,
			stone,
			player,
		}
	}
}
