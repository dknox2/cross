use std::collections::{HashMap, HashSet, VecDeque};

use crate::map::Map;
use crate::point::Point;

// TODO Need to organize my monster spawning and movement logic better.
pub fn find_shortest_path_to(
	map: &Map,
	start: &Point,
	end: &Point,
) -> Vec<Point> {
	let mut queue = VecDeque::new();
	let mut explored = HashSet::new();
	let mut parents = HashMap::new();

	explored.insert(*start);
	queue.push_back(*start);

	while !queue.is_empty() {
		let current = queue.pop_front().expect("No element found.");
		if current == *end {
			return backtrace(&parents, start, end);
		}

		let mut adjacent_tiles = get_traversible_adjacent_tiles(map, current.x, current.y);

		for tile in adjacent_tiles {
			if !explored.contains(&tile) {
				queue.push_back(tile);
				explored.insert(tile);
				parents.insert(tile, current);
			}
		}
	}

	Vec::new()
}

fn get_traversible_adjacent_tiles(map: &Map, x: i32, y: i32) -> Vec<Point> {
	let mut adjacent_indices = Vec::new();

	for new_x in x - 1..=x + 1 {
		for new_y in y - 1..=y + 1 {
			let index = map.coordinates_to_index(new_x, new_y);
			if map.tiles[index].is_traversible() {
				let position = Point { x: new_x, y: new_y };
				adjacent_indices.push(position);
			}
		}
	}

	adjacent_indices
}

fn backtrace(parents: &HashMap<Point, Point>, start: &Point, end: &Point) -> Vec<Point> {
	let mut path = Vec::new();
	path.push(*end);

	while path.last().expect("No element found.") != start {
		let next = parents[path.last().expect("No element found.")];
		path.push(next);
	}

	path.reverse();
	path
}
