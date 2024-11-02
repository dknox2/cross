use crate::entity::Entity;

pub struct CreatureInfo {
	pub entity: Entity,
	pub max_health: i32,
	pub health: i32,
	pub strength: i32,
}
