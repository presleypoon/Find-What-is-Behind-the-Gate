use crate::ASSETS;
use include_dir::File;
use macroquad::prelude::*;
use colored::*;

pub struct Texture {
	pub dirt: Texture2D,
	pub grass: Texture2D,
	pub stone: Texture2D,
	pub gate_locked: Texture2D,
	pub gate_unlocked: Texture2D,
	pub player: [Texture2D; 9],
	pub key: [Texture2D; 1],
}
impl Texture {
	pub async fn new() -> Self {
		let dirt: Texture2D = Self::load_block("dirt");
		let grass: Texture2D = Self::load_block("grass");
		let stone: Texture2D = Self::load_block("stone");
		let gate_locked: Texture2D = Self::load_block("gate_locked");
		let gate_unlocked: Texture2D = Self::load_block("gate_unlocked");
		let player: [Texture2D; 9] = Self::load_entity("player");
		let key: [Texture2D; 1] = Self::load_entity("key");

		Self {dirt, grass, stone, gate_locked, gate_unlocked, player, key}
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

		return_vec.try_into().unwrap_or_else(|_| -> [Texture2D; N] { unreachable!("Can't have less item in the list unless there's a iterate in the previous for loop, added 1 to i but not appended to the vec") })
	}

	fn load_img(path: &str) -> Texture2D {
		let tnf_file: &File<'_> = ASSETS.get_file("textures/tnf.png").expect("Can't find TNF");

		Texture2D::from_image(
			&Image::from_file_with_format(
				ASSETS
					.get_file(path)
					.unwrap_or_else(|| -> &File<'_> {
						eprintln!("{}", format!("Can't read content of {}", path).red());
						tnf_file
					})
					.contents(),
				Some(ImageFormat::Png),
			)
			.unwrap_or_else(|_| -> Image {
				Image::from_file_with_format(tnf_file.contents(), Some(ImageFormat::Png))
					.unwrap_or_else(|e| -> Image { panic!("Can't decode textures/tnf.png with {}", e) })
			}),
		)
	}
}
