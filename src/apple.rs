use crate::snek::*;
use crate::snekr::*;
use rand::Rng;

fn random_mod_20() -> i32 {
    rand::thread_rng().gen_range(0..600 / RECT_WIDTH) * RECT_WIDTH
}

pub struct Apple {
    pub pos: (i32, i32),
}

impl Apple {
    pub fn new(s: &Snek) -> Apple {
        let mut apple = Apple { pos: (0, 0) };
        apple.update(s);
        apple
    }
    pub fn update(&mut self, s: &Snek) {
        let mut x = random_mod_20();
        let mut y = random_mod_20();
        while s.body.iter().any(|r| r.pos.0 == x || r.pos.1 == y) {
            x = random_mod_20();
            y = random_mod_20();
        }
        self.pos.0 = x;
        self.pos.1 = y;
    }
}
