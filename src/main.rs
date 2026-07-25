mod player;
mod world;
mod texture;
mod render;
use player::*;
use world::*;
use texture::*;
use render::*;

use macroquad::prelude::*;

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

