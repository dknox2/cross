use crate::point::Point;

pub struct Creature {
    pub position: Point,
    pub name: String,
    pub max_health: i32,
    pub health: i32,
    pub strength: i32,
}
