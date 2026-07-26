mod player;
mod render;
mod texture;
mod world;
use player::*;
use render::*;
use texture::*;
use world::*;

use macroquad::prelude::*;
use std::{
	fs::read_to_string,
	time::{Duration, Instant},
};

const TPS: f32 = 60.0;

fn window_conf() -> Conf {
	Conf {
		window_title: "Find What is Behind the Gate".to_string(),
		window_width: 800,
		window_height: 600,
		window_resizable: false,
		// icon: (),
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	println!("Game Starts");

	let tick_rate: Duration = Duration::from_secs_f32(1.0 / TPS);
	let mut last_tick: Instant = Instant::now();
	let mut accumlator: Duration = Duration::ZERO;
	let running: bool = true;
	println!("Time stuff init suc.");

	let texture: Texture = Texture::new().await;
	println!("Texture init suc.");

	let mut player: Player = Player::new();
	println!("Player init suc.");

	let world: World =
		World::load_world(read_to_string("assets/level/level_1.txt").expect("Can't find level 1"));
	println!("World init suc.");

	build_textures_atlas();
	println!("Builded atlas");

	loop {
		if is_key_pressed(KeyCode::Escape) {
			println!("ESC detected, program's ending");
			break;
		}

		let elapsed = last_tick.elapsed();
		last_tick = Instant::now();
		accumlator += elapsed;

		if running {
			while accumlator >= tick_rate {
				player.r#move();
				accumlator -= tick_rate
			}
		} else {
			accumlator = Duration::ZERO;
		}

		render(&player, &world, &texture);

		next_frame().await;
	}

	println!("Game exited");
}
