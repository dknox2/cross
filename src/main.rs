mod creature_info;
mod entity;
mod game;
mod map;
mod monster;
mod pathfinding;
mod player;
mod point;
mod rect;
mod tui;

use std::io::{stdin, stdout, Write};

use crate::creature_info::CreatureInfo;
use crate::entity::Entity;
use crate::game::Game;
use crate::point::Point;

use crossterm::event::{read, Event, KeyCode};

// Not sure if this is an especially good way to handle things.
// Need to draw out what I think the interfaces and such should actually look like.
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
	let mut game = Game::new();

	tui::setup_terminal()?;
	tui::draw_map(&game.map)?;
	tui::draw_entity(&game.player.creature_info.entity)?;
	tui::draw_monsters(&game.monsters)?;
	tui::draw_hud(&game)?;

	stdout().flush()?;

	let mut game_alive = true;
	'main_loop: while game_alive {
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

		game.delete_dead_monsters();

		game.move_monsters();

		tui::draw_map(&game.map)?;
		tui::draw_entity(&game.player.creature_info.entity)?;
		tui::draw_monsters(&game.monsters)?;
		tui::draw_hud(&game)?;
		
		if game.player.creature_info.health <= 0 {
			game_alive = false;
		}

		stdout().flush()?;
	}

	if !game_alive {
		tui::draw_game_over_screen()?;
		stdout().flush()?;
		
		loop {
			let event = read()?;
			// TODO More elegant way to do this?
			if let Event::Key(event) = event {
				if event.code == KeyCode::Enter {
					break;
				}
			}
		}
	}

	tui::teardown_terminal()?;

	for monster in &game.monsters {
		println!("{}", monster.creature_info.health);
	}

	Ok(())
}
