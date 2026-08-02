mod entity;
mod player;
mod render;
mod texture;
mod world;
use player::*;
use render::*;
use texture::*;
use world::*;

use include_dir::{Dir, include_dir};
use macroquad::prelude::*;
use std::time::{Duration, Instant};

const TPS: f32 = 60.0;

static ASSETS: Dir = include_dir!("assets");

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

	let mut world: World = World::load_world();
	println!("World init suc.");

	build_textures_atlas();
	println!("Builded atlas");

	loop {
		if is_key_pressed(KeyCode::Escape) {
			println!("ESC detected, program's ending");
			break;
		}

		let elapsed: Duration = last_tick.elapsed();
		last_tick = Instant::now();
		accumlator += elapsed;

		if running {
			while accumlator >= tick_rate {
				player.r#move(&world);
				world.entities_update(&player);
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
