mod creature;
mod game;
mod map;
mod player;
mod point;
mod rect;
mod tui;

use std::io::{stdout, Write};

use crate::creature::Creature;
use crate::game::Game;
use crate::map::TileType;
use crate::point::Point;

use crossterm::event::{read, Event, KeyCode};

fn main() -> std::io::Result<()> {
    let mut random = rand::thread_rng();
    let map = map::Map::with_rooms_and_corridors(&mut random, 1);
    let player_i = map.rooms[0].x1 + 1;
    let player_j = map.rooms[0].y1 + 1;

    let position = Point {
        x: player_i,
        y: player_j,
    };
    let creature = Creature {
        position,
        name: String::from("Player"),
        max_health: 12,
        health: 12,
        strength: 1,
    };

    let player = player::Player { creature };

	let mut game = Game { map, player, monsters: Vec::new() };
	game.spawn_monsters_in_map_rooms();

    tui::setup_terminal()?;
    tui::draw_map(&game.map)?;
    tui::draw_player(&game.player)?;
	tui::draw_monsters(&game.monsters)?;

    stdout().flush()?;

    'main_loop: loop {
        let event = read()?;
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Up => {
                    let coordinates = (game.player.creature.position.x, game.player.creature.position.y - 1);
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
                        game.player.creature.position.make_move(0, -1);
                    }
                }
                KeyCode::Down => {
                    let coordinates = (game.player.creature.position.x, game.player.creature.position.y + 1);
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
                        game.player.creature.position.make_move(0, 1);
                    }
                }
                KeyCode::Left => {
                    let coordinates = (game.player.creature.position.x - 1, game.player.creature.position.y);
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
                        game.player.creature.position.make_move(-1, 0);
                    }
                }
                KeyCode::Right => {
                    let coordinates = (game.player.creature.position.x + 1, game.player.creature.position.y);
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
                        game.player.creature.position.make_move(1, 0);
                    }
                }
                KeyCode::Esc => break 'main_loop,
                _ => {}
            }
        }

		for monster in &mut game.monsters {
			let path = game.map.find_shortest_path_to(&monster.position, &game.player.creature.position);
			// TODO We should maybe find a better way to handle when a path cannot be found.
			if path.len() > 1 {
				let next_step = path[1];
				monster.position.x = next_step.x;
				monster.position.y = next_step.y;
			}
		}

        tui::draw_map(&game.map)?;
        tui::draw_player(&game.player)?;
		tui::draw_monsters(&game.monsters)?;
        stdout().flush()?;
    }

    tui::teardown_terminal()?;

    Ok(())
}
