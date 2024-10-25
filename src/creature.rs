use crate::position::Position;

pub struct Creature {
    pub position: Position,
    pub name: String,
    pub max_health: i32,
    pub health: i32,
    pub strength: i32,
}
