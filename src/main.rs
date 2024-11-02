mod creature_info;
mod entity;
mod game;
mod map;
mod monster;
mod player;
mod point;
mod rect;
mod tui;

use std::io::{stdout, Write};

use crate::creature_info::CreatureInfo;
use crate::entity::Entity;
use crate::game::Game;
use crate::point::Point;

use crossterm::event::{read, Event, KeyCode};

fn move_player(game: &mut Game, direction: &Point) {
	let destination = Point {
		x: game.player.creature_info.entity.position.x + direction.x,
		y: game.player.creature_info.entity.position.y + direction.y,
	};
	let index = game.map.point_to_index(&destination);
	if game.map.tiles[index].is_traversible() {
		game.move_player(destination);
	}
}

fn main() -> std::io::Result<()> {
	let mut random = rand::thread_rng();
	let map = map::Map::with_rooms_and_corridors(&mut random, 1);
	let player_i = map.rooms[0].x1 + 1;
	let player_j = map.rooms[0].y1 + 1;

	let position = Point {
		x: player_i,
		y: player_j,
	};
	let player_entity = Entity {
		name: String::from("Player"),
		glyph: '@',
		position,
	};
	let creature_info = CreatureInfo {
		entity: player_entity,
		max_health: 12,
		health: 12,
		strength: 1,
	};

	let player = player::Player { creature_info };

	let mut game = Game {
		map,
		player,
		monsters: Vec::new(),
	};
	game.spawn_monsters_in_map_rooms();

	tui::setup_terminal()?;
	tui::draw_map(&game.map)?;
	tui::draw_entity(&game.player.creature_info.entity)?;
	tui::draw_monsters(&game.monsters)?;
	tui::draw_hud(&game)?;

	stdout().flush()?;

	'main_loop: loop {
		let event = read()?;
		if let Event::Key(event) = event {
			match event.code {
				KeyCode::Up | KeyCode::Char('8') => {
					let direction = Point { x: 0, y: -1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Down | KeyCode::Char('2') => {
					let direction = Point { x: 0, y: 1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Left | KeyCode::Char('4') => {
					let direction = Point { x: -1, y: 0 };
					move_player(&mut game, &direction);
				}
				KeyCode::Right | KeyCode::Char('6') => {
					let direction = Point { x: 1, y: 0 };
					move_player(&mut game, &direction);
				}
				KeyCode::Char('7') => {
					let direction = Point { x: -1, y: -1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Char('9') => {
					let direction = Point { x: 1, y: -1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Char('1') => {
					let direction = Point { x: -1, y: 1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Char('3') => {
					let direction = Point { x: 1, y: 1 };
					move_player(&mut game, &direction);
				}
				KeyCode::Char('5') => {}
				KeyCode::Esc => break 'main_loop,
				_ => {
					continue;
				}
			}
		}

		game.move_monsters();

		tui::draw_map(&game.map)?;
		tui::draw_entity(&game.player.creature_info.entity)?;
		tui::draw_monsters(&game.monsters)?;
		tui::draw_hud(&game)?;
		
		stdout().flush()?;
	}

	tui::teardown_terminal()?;

	for monster in &game.monsters {
		println!("{}", monster.creature_info.health);
	}

	Ok(())
}
