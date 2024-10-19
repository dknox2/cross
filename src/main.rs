mod game;
mod map;
mod player;
mod rect;
mod tui;

use crate::map::TileType;

use crossterm::event::{read, Event, KeyCode};

fn main() -> std::io::Result<()> {
	let mut random = rand::thread_rng();
	let map = map::Map::with_rooms_and_corridors(&mut random, 1);
	let player_i = map.rooms[0].x1 + 1;
	let player_j = map.rooms[0].y1 + 1;

	let mut player = player::Player { i: player_i, j: player_j };
	tui::setup_terminal()?;
	tui::draw_map(&map)?;
	tui::draw_player(&player)?;

	'main_loop: loop {
		let event = read()?;
		if let Event::Key(event) = event {
			match event.code {
				// TODO Are we handling these inputs backwards?
				KeyCode::Up => {
					let coordinates = (player.i - 1, player.j);
					let index = map.coordinates_to_index(coordinates.0, coordinates.1);
					if map.tiles[index] != TileType::Wall {
						player.i -= 1;
					}
				},
				KeyCode::Down => {
					player.i = player.i + 1;
				},
				KeyCode::Left => {
					player.j = player.j - 1;
				},
				KeyCode::Right => {
					player.j = player.j + 1;
				},
				KeyCode::Esc => break 'main_loop,
				_ => {}
			}
		}

		tui::draw_map(&map)?;
		tui::draw_player(&player)?;
	}

	tui::teardown_terminal()?;

	Ok(())
}
