use crate::preview::draw;

mod game;
mod preview;
mod util;

const SEED: i32 = 4;

#[macroquad::main("Map Preview")]
async fn main() {
    draw();
}
