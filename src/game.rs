use std::collections::HashSet;

use rand::thread_rng;

use crate::creature_info::CreatureInfo;
use crate::entity::Entity;
use crate::map::Map;
use crate::monster::Monster;
use crate::pathfinding::find_shortest_path_to;
use crate::player::Player;
use crate::point::Point;

pub struct Game {
	pub map: Map,
	pub player: Player,
	pub monsters: Vec<Monster>,
}

impl Game {
	pub fn spawn_monsters_in_map_rooms(&mut self) {
		for room in &self.map.rooms {
			let center = room.center();
			let point = Point {
				x: center.0,
				y: center.1,
			};
			let goblin_entity = Entity {
				name: String::from("Goblin"),
				glyph: 'g',
				position: point,
			};
			let goblin_info = CreatureInfo {
				entity: goblin_entity,
				max_health: 5,
				health: 5,
				strength: 1,
			};
			let goblin = Monster {
				creature_info: goblin_info,
			};

			self.monsters.push(goblin);
		}
	}

	pub fn move_monsters(&mut self) {
		let mut current_positions = self
			.monsters
			.iter()
			.map(|monster| monster.creature_info.entity.position)
			.collect::<HashSet<Point>>();
		for monster in &mut self.monsters {
			// TODO It could be very good to just keep an rng field and re-use that.
			let mut rng = thread_rng();
			let path = find_shortest_path_to(
				&self.map,
				&monster.creature_info.entity.position,
				&self.player.creature_info.entity.position
			);
			if path.len() > 1 {
				let next_step = path[1];
				let mut empty_destination = !current_positions.contains(&next_step);
				if self.player.creature_info.entity.position == next_step {
					empty_destination = false;
					self.player.creature_info.health -= monster.creature_info.strength;
				}
				if empty_destination {
					current_positions.remove(&monster.creature_info.entity.position);
					current_positions.insert(next_step);
					monster.creature_info.entity.position = next_step;
				}
			}
		}
	}

	pub fn move_player(&mut self, destination: Point) {
		let mut empty_destination = true;
		for monster in &mut self.monsters {
			if monster.creature_info.entity.position == destination {
				empty_destination = false;
				monster.creature_info.health -= self.player.creature_info.strength;
			}
		}
		if empty_destination {
			self.player.creature_info.entity.position = destination;
		}
	}
}
