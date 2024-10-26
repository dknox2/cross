use crate::point::Point;

pub struct Creature {
	pub position: Point,
	pub name: String,
	pub max_health: i32,
	pub health: i32,
	pub strength: i32,
}

impl Creature {
	pub fn move_or_attack(&mut self, direction_vector: &Point, other_creatures: &mut [Creature]) {
		let destination = Point {
			x: self.position.x + direction_vector.x,
			y: self.position.y + direction_vector.y,
		};

		let mut empty_destination = true;
		for monster in other_creatures.iter_mut() {
			if monster.position == destination {
				empty_destination = false;
				monster.health -= self.strength;
			}
		}
		if empty_destination {
			self.position.x += direction_vector.x;
			self.position.y += direction_vector.y;
		}
	}
}
