mod ship;

use std::num::FpCategory::Nan;
use std::thread::sleep;
use macroquad::color::{BLACK, BLUE, GREEN, ORANGE, PINK, PURPLE, RED, WHITE, YELLOW};
use macroquad::input::{is_key_pressed, mouse_position, KeyCode};
use macroquad::prelude::{clear_background, is_key_down};
use macroquad::text::draw_text;
use macroquad::window::{next_frame, screen_height, screen_width};
use crate::ship::{inputs_handler, Ship};

#[macroquad::main("shipGame")]
async fn main() {
    let mut ship = Ship::new(100.0, 100.0, 90.0, WHITE);
    let mut follower = Ship::new(200.0, 200.0, 90.0, RED);
    let mut follower2 = Ship::new(300.0, 300.0, 90.0, PINK);
    let mut follower3 = Ship::new(400.0, 400.0, 90.0, ORANGE);
    let mut follower4 = Ship::new(500.0, 500.0, 90.0, YELLOW);
    let mut follower5 = Ship::new(600.0, 600.0, 90.0, GREEN);
    let mut follower6 = Ship::new(700.0, 700.0, 90.0, BLUE);
    let mut follower7 = Ship::new(800.0, 800.0, 90.0, PURPLE);

    clear_background(BLACK);
    loop {
        inputs_handler(&mut ship);

        follower.move_towards_mouse(20.0, 30.0);
        follower2.move_towards_mouse(18.0, 35.0);
        follower3.move_towards_mouse(16.0, 40.0);
        follower4.move_towards_mouse(14.0, 45.0);
        follower5.move_towards_mouse(12.0, 50.0);
        follower6.move_towards_mouse(10.0, 55.0);
        follower7.move_towards_mouse(8.0, 60.0);

        ship.draw();
        follower7.draw();
        follower6.draw();
        follower5.draw();
        follower4.draw();
        follower3.draw();
        follower2.draw();
        follower.draw();
        next_frame().await;
    }
}
