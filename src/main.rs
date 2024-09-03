mod apple;
mod snek;
mod snekr;

use apple::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use snek::*;
use snekr::*;

const DIM: i32 = 800;

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, snek: &Snek, apple: &Apple) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::WHITE);

    snek.body
        .iter()
        .for_each(|i| d.draw_rectangle(i.pos.0, i.pos.1, RECT_WIDTH, RECT_WIDTH, Color::GREEN));
    d.draw_rectangle(apple.pos.0, apple.pos.1, RECT_WIDTH, RECT_WIDTH, Color::RED)
}

fn update(rl: &mut RaylibHandle, snek: &mut Snek) {
    if let Some(new_dir) = match rl.get_key_pressed() {
        Some(KEY_W) if snek.dir != Directon::DOWN => Some(Directon::UP),
        Some(KEY_A) if snek.dir != Directon::RIGHT => Some(Directon::LEFT),
        Some(KEY_S) if snek.dir != Directon::UP => Some(Directon::DOWN),
        Some(KEY_D) if snek.dir != Directon::LEFT => Some(Directon::RIGHT),
        _ => None,
    } {
        snek.dir = new_dir;
    }
    snek.move_snek();
}

fn is_game_over(snek: &Snek) -> bool {
    let head = snek.head();
    head.0 < 0 || head.0 + RECT_WIDTH > DIM || head.1 < 0 || head.1 + RECT_WIDTH > DIM
}

fn main() {
    let (mut rl, thread) = raylib::init().size(DIM, DIM).title("Snek.rs").build();

    let mut snek = Snek::new(DIM);
    let mut apple = Apple::new(&snek);

    rl.set_target_fps(15);
    while !rl.window_should_close() && !is_game_over(&snek) {
        update(&mut rl, &mut snek);
        snek.handle_apple(&mut apple);
        draw(&mut rl, &thread, &snek, &apple);
    }
}
