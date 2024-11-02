use crate::point::Point;

// TODO We'll need to think of a nice way to handle items too.
pub struct Entity {
	pub name: String,
	pub glyph: char,
	pub position: Point,
}
