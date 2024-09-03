use crate::{snekr::*, Apple};

#[derive(PartialEq)]
pub enum Directon {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snek {
    pub body: Vec<SnekR>,
    pub dir: Directon,
}

impl Snek {
    pub fn new(dim: i32) -> Self {
        Snek {
            body: vec![SnekR {
                pos: (dim / 2, dim / 2),
            }],
            dir: Directon::UP,
        }
    }
    fn append_body(&mut self) {
        let last = self.body.last().expect("UNREACHABLE");
        self.body.push(SnekR { pos: last.pos });
    }
    pub fn head(&self) -> (i32, i32) {
        if let Some(head) = self.body.get(0) {
            head.pos
        } else {
            panic!("UNREACHABLE")
        }
    }
    fn get_scalar(&self) -> (i32, i32) {
        match self.dir {
            Directon::UP => (0, -1),
            Directon::LEFT => (-1, 0),
            Directon::DOWN => (0, 1),
            Directon::RIGHT => (1, 0),
        }
    }

    pub fn move_snek(&mut self) {
        let scalar = self.get_scalar();
        for i in (1..self.body.len()).rev() {
            self.body.get_mut(i).expect("wtf").pos = self.body.get(i - 1).expect("UNREACHABLE").pos;
        }
        self.body
            .get_mut(0)
            .expect("UNREACHABLE")
            .add_scalar(scalar)
    }

    pub fn handle_apple(&mut self, apple: &mut Apple) {
        let head = self.head();
        if head.0 == apple.pos.0 && head.1 == apple.pos.1 {
            self.append_body();
            apple.update(self);
        }
    }
}
