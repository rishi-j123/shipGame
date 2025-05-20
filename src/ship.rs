use macroquad::input::KeyCode::Backspace;
use macroquad::prelude::*;

pub const SHIP_LENGTH_TO_TIP: f32 = 12.5;
pub const SHIP_LENGTH_TO_REAR: f32 = 12.5;
pub const SHIP_WIDTH: f32 = 5.0;
pub const SHIP_WING_OFFSET: f32 = 135.0;

pub struct Ship {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub color: Color,
    pub trail_bool: bool,
    trail_vec: Vec<(f32, f32)>,
    pub trail_color: Color,
    pub trail_weight: f32,
}

pub struct OffsetList {
    tx: f32,
    ty: f32,
    blx: f32,
    bly: f32,
    brx: f32,
    bry: f32,
}

impl Ship {
    pub fn new(x: f32, y: f32, angle: f32, color: Color, trail: bool, trail_color: Color, trail_weight: f32) -> Self {
        Self{
            x,
            y,
            angle,
            color,
            trail_bool: trail,
            trail_vec: Vec::<(f32,f32)>::new(),
            trail_color,
            trail_weight,
        }
    }

    pub fn get_offsets(&self) -> OffsetList {
        //tip offset calculation
        let hyp = f32::sqrt(SHIP_LENGTH_TO_REAR.powf(2.0) + (SHIP_WIDTH/2.0).powf(2.0));

        // https://www.desmos.com/calculator/tjz1yuwvpv
        let tx = SHIP_LENGTH_TO_TIP*self.angle.to_radians().cos();
        let ty = SHIP_LENGTH_TO_TIP*self.angle.to_radians().sin();
        let blx = hyp*f32::cos(self.angle.to_radians()+SHIP_WING_OFFSET.to_radians());
        let bly = hyp*f32::sin(self.angle.to_radians()+SHIP_WING_OFFSET.to_radians());
        let brx = hyp*f32::cos(self.angle.to_radians()-SHIP_WING_OFFSET.to_radians());
        let bry = hyp*f32::sin(self.angle.to_radians()-SHIP_WING_OFFSET.to_radians());

        OffsetList{
            tx,
            ty,
            blx,
            bly,
            brx,
            bry,
        }
    }

    pub fn draw(&self) {
        if self.trail_bool {
            for dot in self.trail_vec.iter() {
                draw_circle(dot.0, dot.1, self.trail_weight, self.trail_color);
            }
        }

        let offsets = self.get_offsets();

        // -y due to y-axis being flipped: y0 is at top of screen
        let tip = Vec2::new(self.x + offsets.tx, self.y - offsets.ty);
        let back_left = Vec2::new(self.x + offsets.blx, self.y - offsets.bly);
        let back_right = Vec2::new(self.x + offsets.brx, self.y - offsets.bry);
        draw_triangle(tip, back_left, back_right, self.color);
    }

    pub fn add_trail_dot(&mut self) {
        self.trail_vec.push((self.x, self.y));
    }

    pub fn turn_angle(&mut self, angle: f32) {
        self.angle += angle;
    }

    pub fn move_forward(&mut self, distance: f32) {
        let distance_x = distance*f32::cos(self.angle.to_radians());
        let distance_y = distance*f32::sin(self.angle.to_radians());

        self.x = self.x + distance_x;
        self.y = self.y - distance_y;
    }

    pub fn move_towards_mouse(&mut self, distance: f32, limit: f32) {
        if self.dist_from_mouse() > limit {
            let delta_angle = self.mouse_delta_angle();
            self.turn_angle(delta_angle);
            self.move_forward(distance);
        }
    }

    pub fn dist_from_mouse(&self) -> f32 {
        let mouse_pos = mouse_position();
        let dx = mouse_pos.0 - self.x;
        let dy = -(mouse_pos.1 - self.y);
        f32::sqrt(dx.powf(2.0) + dy.powf(2.0))
    }

    pub fn mouse_delta_angle(&mut self) -> f32 {
        let mouse_pos = mouse_position();
        let dx = mouse_pos.0 - self.x;
        let dy = -(mouse_pos.1 - self.y);

        let mut target_angle = f32::atan2(dy,dx).to_degrees();
        if target_angle < 0.0 {
            target_angle = 180.0 + (180.0 + target_angle);
        }
        if target_angle == 360.0 {
            target_angle = 0.0;
        }

        target_angle - self.angle
    }
}

pub fn inputs_handler(ship: &mut Ship, turn_amount: f32, forward_dist: f32) {
    if is_key_pressed(KeyCode::Space) {
        ship.x = screen_width() / 2.0;
        ship.y = screen_height() / 2.0;
    }
    if is_key_down(KeyCode::A) {
        ship.turn_angle(turn_amount);
    }
    if is_key_down(KeyCode::D) {
        ship.turn_angle(-turn_amount);
    }
    if is_key_down(KeyCode::W) {
        ship.move_forward(forward_dist);
    }
    if is_key_down(KeyCode::Enter) {
        ship.add_trail_dot();
    }
    if is_key_down(Backspace) {
        ship.trail_vec = Vec::new();
    }
}