mod ship;

use macroquad::color::{BLACK, WHITE};
use macroquad::input::KeyCode;
use macroquad::prelude::{clear_background, is_key_down};
use macroquad::text::draw_text;
use macroquad::window::next_frame;
use crate::ship::Ship;

#[macroquad::main("shipGame")]
async fn main() {
    let mut ship = Ship::new(100.0, 100.0, 0.0, WHITE);
    clear_background(BLACK);
    loop {
        if is_key_down(KeyCode::A) {
            ship.turn_angle(5.0);
        }
        if is_key_down(KeyCode::D) {
            ship.turn_angle(-5.0);
        }
        if is_key_down(KeyCode::W) {
            ship.move_forward(5.0);
        }

        ship.move_towards_mouse(10.0);

        ship.draw();
        next_frame().await;
    }
}
