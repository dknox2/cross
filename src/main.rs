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

    let mut game = Game {
        map,
        player,
        monsters: Vec::new(),
    };
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
                // TODO We should move the "is valid move" logic to the move method in Game
                KeyCode::Up | KeyCode::Char('8') => {
                    let direction = Point { x: 0, y: -1 };
                    let coordinates = Point {
                        x: game.player.creature.position.x,
                        y: game.player.creature.position.y - 1,
                    };
                    let index = game.map.point_to_index(&coordinates);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
                }
                KeyCode::Down | KeyCode::Char('2') => {
					let direction = Point { x: 0, y: 1 };
                    let coordinates = (
                        game.player.creature.position.x,
                        game.player.creature.position.y + 1,
                    );
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
                }
                KeyCode::Left | KeyCode::Char('4') => {
					let direction = Point { x: -1, y: 0 };
                    let coordinates = (
                        game.player.creature.position.x - 1,
                        game.player.creature.position.y,
                    );
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
                }
                KeyCode::Right | KeyCode::Char('6') => {
					let direction = Point { x: 1, y: 0 };
                    let coordinates = (
                        game.player.creature.position.x + 1,
                        game.player.creature.position.y,
                    );
                    let index = game.map.coordinates_to_index(coordinates.0, coordinates.1);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
                }
				KeyCode::Char('7') => {
                    let direction = Point { x: -1, y: -1 };
                    let coordinates = Point {
                        x: game.player.creature.position.x,
                        y: game.player.creature.position.y - 1,
                    };
                    let index = game.map.point_to_index(&coordinates);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
				}
				KeyCode::Char('9') => {
                    let direction = Point { x: 1, y: -1 };
                    let coordinates = Point {
                        x: game.player.creature.position.x,
                        y: game.player.creature.position.y - 1,
                    };
                    let index = game.map.point_to_index(&coordinates);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
				}
				KeyCode::Char('1') => {
                    let direction = Point { x: -1, y: 1 };
                    let coordinates = Point {
                        x: game.player.creature.position.x,
                        y: game.player.creature.position.y - 1,
                    };
                    let index = game.map.point_to_index(&coordinates);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
                    }
				}
				KeyCode::Char('3') => {
                    let direction = Point { x: 1, y: 1 };
                    let coordinates = Point {
                        x: game.player.creature.position.x,
                        y: game.player.creature.position.y - 1,
                    };
                    let index = game.map.point_to_index(&coordinates);
                    if game.map.tiles[index].is_traversible() {
						game.player.creature.move_or_attack(&direction, &mut game.monsters);
					}
				}
				KeyCode::Char('5') => { }
                KeyCode::Esc => break 'main_loop,
                _ => {}
            }
        }

        for monster in &mut game.monsters {
            let path = game
                .map
                .find_shortest_path_to(&monster.position, &game.player.creature.position);
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

	for monster in &game.monsters {
		println!("{}", monster.health);
	}

    Ok(())
}
