use macroquad::color::Color;
use crate::ship::Ship;

pub struct ShipInitiator {
    fleet: Vec<Ship>,
}

impl ShipInitiator {
    pub fn new(count: u32, x: f32, y:f32, angle: f32, color_pattern: Vec<Color>) -> Self {
        let mut fleet: Vec<Ship> = Vec::new();
        let color_vec = generate_color_list(count as usize, color_pattern);
        for i in 0..count {
            fleet.push(Ship::new(x, y, angle, color_vec[i as usize]));
        }

        ShipInitiator {
            fleet
        }
    }

    pub fn fleet_follow_mouse(&mut self, distance: f32, limit: f32, dist_stagger: f32, limitStagger: f32) {
        let mut count = 0.0;
        for mut ship in &mut self.fleet {
            let limitOffset = limitStagger * count;
            let distOffset = dist_stagger * count;
            ship.move_towards_mouse(distance - distOffset, limit + limitOffset);
            count += 1.0;
        }
    }

    pub fn draw(&mut self) {
        let mut count= 0;
        for mut ship in &mut self.fleet {
            count += 1;
            ship.draw();
        }
    }
}

pub fn generate_color_list(target_length: usize, input_colors: Vec<Color>) -> Vec<Color> {
    let mut colors = Vec::new();
    let mut input_colors_length = input_colors.len();
    let mut index = 0;

    loop {
        if colors.len() == target_length {
            return colors;
        }
        colors.push(input_colors[index]);
        index += 1;
        if index == input_colors.len() {
            index = 0;
        }
    }
}