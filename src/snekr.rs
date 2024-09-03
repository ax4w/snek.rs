pub const RECT_WIDTH: i32 = 20;

pub struct SnekR {
    pub pos: (i32, i32),
}

impl SnekR {
    pub fn add_scalar(&mut self, scalar: (i32, i32)) {
        self.pos.0 += scalar.0 * RECT_WIDTH;
        self.pos.1 += scalar.1 * RECT_WIDTH;
    }
}
