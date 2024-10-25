#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn make_move(&mut self, x_change: i32, y_change: i32) {
        self.x += x_change;
        self.y += y_change;
    }
}
