use crate::ASSETS;
use macroquad::prelude::*;

pub struct Texture {
	pub dirt: Texture2D,
	pub grass: Texture2D,
	pub stone: Texture2D,
	pub player: [Texture2D; 9],
}
impl Texture {
	pub async fn new() -> Self {
		let dirt = Self::load_block("dirt");
		let grass = Self::load_block("grass");
		let stone = Self::load_block("stone");
		let player = Self::load_entity("player");

		Self {
			dirt,
			grass,
			stone,
			player,
		}
	}

	fn load_block(name: &str) -> Texture2D {
		Self::load_img(&format!("textures/blocks/{}.png", name))
	}

	fn load_entity<const N: usize>(name: &str) -> [Texture2D; N] {
		let mut return_vec: Vec<Texture2D> = Vec::new();

		for i in 0..N {
			return_vec.push(Self::load_img(&format!(
				"textures/entity/{}/{}.png",
				name, i
			)))
		}

		return_vec.try_into().unwrap_or_else(|_| unreachable!("Can't have less item in the list unless there's a iterate in the previous for loop, added 1 to i but not appended to the vec"))
	}

	fn load_img(path: &str) -> Texture2D {
		let tnf_file = ASSETS.get_file("textures/tnf.png").expect("Can't find TNF");

		Texture2D::from_image(
			&Image::from_file_with_format(
				ASSETS
					.get_file(path)
					.unwrap_or_else(|| {
						eprintln!("Can't read content of {}", path);
						tnf_file
					})
					.contents(),
				Some(ImageFormat::Png),
			)
			.unwrap_or_else(|_| {
				Image::from_file_with_format(tnf_file.contents(), Some(ImageFormat::Png))
					.unwrap_or_else(|e| panic!("Can't decode textures/tnf.png with {}", e))
			}),
		)
	}
}
