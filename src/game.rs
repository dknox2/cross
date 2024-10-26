use crate::creature::Creature;
use crate::map::Map;
use crate::player::Player;
use crate::point::Point;

pub struct Game {
	pub map: Map,
	pub player: Player,
	pub monsters: Vec<Creature>,
}

impl Game {
	pub fn spawn_monsters_in_map_rooms(&mut self) {
		for room in &self.map.rooms {
			let center = room.center();
			let point = Point {
				x: center.0,
				y: center.1,
			};
			let goblin = Creature {
				position: point,
				name: "Goblin".to_string(),
				max_health: 5,
				health: 5,
				strength: 1,
			};

			self.monsters.push(goblin);
		}
	}

	pub fn move_player_by(&mut self, direction_vector: &Point) {
		let destination = Point {
			x: self.player.creature.position.x + direction_vector.x,
			y: self.player.creature.position.y + direction_vector.y,
		};

		let mut empty_destination = true;
		for monster in &mut self.monsters {
			if monster.position == destination {
				empty_destination = false;
				monster.health -= self.player.creature.strength;
			}
		}
		if empty_destination {
			self.player.creature.position.x += direction_vector.x;
			self.player.creature.position.y += direction_vector.y;
		}
	}
}
