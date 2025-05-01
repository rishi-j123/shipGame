mod ship;

use std::num::FpCategory::Nan;
use std::thread::sleep;
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{is_key_pressed, mouse_position, KeyCode};
use macroquad::prelude::{clear_background, is_key_down};
use macroquad::text::draw_text;
use macroquad::window::{next_frame, screen_height, screen_width};
use crate::ship::Ship;

#[macroquad::main("shipGame")]
async fn main() {
    let mut ship = Ship::new(100.0, 100.0, 0.0, WHITE);
    println!("before loop: {}", ship.angle);
    clear_background(BLACK);
    loop {
        // println!("mouse pos: {}, {}", mouse_position().0, mouse_position().1);
        if is_key_pressed(KeyCode::Space) {
            ship.x = screen_width() / 2.0;
            ship.y = screen_height() / 2.0;
        }
        if is_key_down(KeyCode::A) {
            ship.turn_angle(5.0);
        }
        if is_key_down(KeyCode::D) {
            ship.turn_angle(-5.0);
        }
        if is_key_down(KeyCode::W) {
            ship.move_forward(10.0);
        }

        ship.mouse_delta_angle();
        // if ship.x.is_nan(){
        //     println!("died");
        // }
        // println!("after call: x: {}, y: {}, angle: {}, \n", ship.x, ship.y, ship.angle);

        ship.draw();
        // sleep(core::time::Duration::from_millis(50));
        next_frame().await;
    }
}
