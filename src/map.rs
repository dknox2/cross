use rand::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::point::Point;
use crate::rect::Rect;

pub const MAP_WIDTH: usize = 60;
pub const MAP_HEIGHT: usize = 20;
pub const MAP_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

impl TileType {
    pub fn is_traversible(&self) -> bool {
        *self != TileType::Wall
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
}

impl Map {
    // TODO Should probably make this take a Point struct.
    // Could be nice to set up variable map width as well.
    pub fn coordinates_to_index(&self, x: i32, y: i32) -> usize {
        (y as usize * MAP_WIDTH) + x as usize
    }

	// TODO We can probably be more memory efficient here.
    fn find_shortest_path_to(&self, start: &Point, end: &Point) -> Vec<Point> {
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();
        let mut parents = HashMap::new();

        explored.insert(*start);
        queue.push_back(*start);

        while !queue.is_empty() {
            let current = queue.pop_front().expect("No element found.");
            if current == *end {
                return self.backtrace(&parents, start, end);
            }
            let adjacent_tiles = self.get_traversible_adjacent_tiles(current.x, current.y);
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

    fn get_traversible_adjacent_tiles(&self, x: i32, y: i32) -> Vec<Point> {
        let mut adjacent_indices = Vec::new();

        for new_x in x - 1..=x + 1 {
            for new_y in y - 1..=y + 1 {
                let index = self.coordinates_to_index(new_x, new_y);
                if self.tiles[index].is_traversible() {
                    let position = Point { x: new_x, y: new_y };
                    adjacent_indices.push(position);
                }
            }
        }

        adjacent_indices
    }

    fn backtrace(
        &self,
        parents: &HashMap<Point, Point>,
        start: &Point,
        end: &Point,
    ) -> Vec<Point> {
        let mut path = Vec::new();
        path.push(*end);

        while path.last().expect("No element found.") != start {
            let next = parents[path.last().expect("No element found.")];
            path.push(next);
        }

        path.reverse();
        path
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let index = self.coordinates_to_index(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, i1: i32, i2: i32, j: i32) {
        for i in min(i1, i2)..=max(i1, i2) {
            let index = self.coordinates_to_index(i, j);
            if index > 0 && index < MAP_COUNT {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, j1: i32, j2: i32, i: i32) {
        for j in min(j1, j2)..=max(j1, j2) {
            let index = self.coordinates_to_index(i, j);
            if index > 0 && index < MAP_COUNT {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn with_rooms_and_corridors(random: &mut ThreadRng, depth: i32) -> Map {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 12;

        let tiles = vec![TileType::Wall; MAP_COUNT];
        let rooms: Vec<Rect> = Vec::new();
        let mut map = Map { tiles, rooms };

        for _ in 0..MAX_ROOMS {
            let width = random.gen_range(MIN_SIZE..MAX_SIZE);
            let height = random.gen_range(MIN_SIZE..MAX_SIZE);
            let i = random.gen_range(1..MAP_WIDTH as i32 - width) - 1;
            let j = random.gen_range(1..MAP_HEIGHT as i32 - height) - 1;

            let new_room = Rect::new(i, j, width, height);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if random.gen::<bool>() {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        let stairs_position = map.rooms[map.rooms.len() - 1].center();
        let stairs_index = map.coordinates_to_index(stairs_position.0, stairs_position.1);
        map.tiles[stairs_index] = TileType::DownStairs;

        map
    }
}
