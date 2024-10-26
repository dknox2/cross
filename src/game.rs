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
			let point = Point { x: center.0, y: center.1 };
			let goblin = Creature {
				position: point,
				name: "Goblin".to_string(),
				max_health: 5,
				health: 5,
				strength: 1
			};

			self.monsters.push(goblin);
		}
	}
}
