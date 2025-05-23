mod ship;
mod initiator;

use macroquad::color::{BLACK, BLUE, GRAY, ORANGE, PINK, PURPLE, RED, SKYBLUE, WHITE, YELLOW};
use macroquad::prelude::{clear_background, GREEN};
use macroquad::window::{next_frame};
use crate::ship::{inputs_handler, Ship};

#[macroquad::main("shipGame")]
async fn main() {
    let mut ship = Ship::new(100.0, 100.0, 90.0, WHITE, true, RED, 2.0);
    let mut fleet = initiator::ShipInitiator::new(
        10,
        100.0,
        100.0,
        0.0,
        vec![RED, PINK, ORANGE, YELLOW, GREEN, SKYBLUE, BLUE, PURPLE, WHITE, GRAY]);

    clear_background(BLACK);
    loop {
        inputs_handler(&mut ship, 10.0, 10.0);
        fleet.fleet_follow_mouse(20.0, 10.0, 0.5, 15.0,);

        ship.draw();
        fleet.draw();
        next_frame().await;
    }
}
