#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    pub fn make_move(&mut self, i_change: i32, j_change: i32) {
        self.i += i_change;
        self.j += j_change;
    }
}
