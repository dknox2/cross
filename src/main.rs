mod creature;
mod game;
mod map;
mod player;
mod point;
mod rect;
mod tui;

use std::io::{stdout, Write};

use crate::creature::Creature;
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

    let mut player = player::Player { creature };
    tui::setup_terminal()?;
    tui::draw_map(&map)?;
    tui::draw_player(&player)?;

    stdout().flush();

    'main_loop: loop {
        let event = read()?;
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Up => {
                    let coordinates = (player.creature.position.x, player.creature.position.y - 1);
                    let index = map.coordinates_to_index(coordinates.0, coordinates.1);
                    if map.tiles[index] != TileType::Wall {
                        player.creature.position.make_move(0, -1);
                    }
                }
                KeyCode::Down => {
                    let coordinates = (player.creature.position.x, player.creature.position.y + 1);
                    let index = map.coordinates_to_index(coordinates.0, coordinates.1);
                    if map.tiles[index] != TileType::Wall {
                        player.creature.position.make_move(0, 1);
                    }
                }
                KeyCode::Left => {
                    let coordinates = (player.creature.position.x - 1, player.creature.position.y);
                    let index = map.coordinates_to_index(coordinates.0, coordinates.1);
                    if map.tiles[index] != TileType::Wall {
                        player.creature.position.make_move(-1, 0);
                    }
                }
                KeyCode::Right => {
                    let coordinates = (player.creature.position.x + 1, player.creature.position.y);
                    let index = map.coordinates_to_index(coordinates.0, coordinates.1);
                    if map.tiles[index] != TileType::Wall {
                        player.creature.position.make_move(1, 0);
                    }
                }
                KeyCode::Esc => break 'main_loop,
                _ => {}
            }
        }

        tui::draw_map(&map)?;
        tui::draw_player(&player)?;
        stdout().flush();
    }

    tui::teardown_terminal()?;

    Ok(())
}
