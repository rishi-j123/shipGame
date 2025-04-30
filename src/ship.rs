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
}

struct OffsetList {
    tx: f32,
    ty: f32,
    blx: f32,
    bly: f32,
    brx: f32,
    bry: f32,
}

impl Ship {
    pub fn new(x: f32, y: f32, angle: f32, color: Color) -> Self {
        Self{
            x,
            y,
            angle,
            color,
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
        let offsets = self.get_offsets();

        // -y due to y-axis being flipped: y0 is at top of screen
        let tip = Vec2::new(self.x + offsets.tx, self.y - offsets.ty);
        let back_left = Vec2::new(self.x + offsets.blx, self.y - offsets.bly);
        let back_right = Vec2::new(self.x + offsets.brx, self.y - offsets.bry);
        draw_triangle(tip, back_left, back_right, self.color);
    }

    pub fn turn_angle(&mut self, angle: f32) {
        self.angle += angle;
        if self.angle >= 360.0 {
            self.angle -= 360.0;
        } else if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        let distance_x = distance*f32::cos(self.angle.to_radians());
        let distance_y = distance*f32::sin(self.angle.to_radians());

        self.x = self.x + distance_x;
        self.y = self.y - distance_y;
    }

    pub fn move_towards_mouse(&mut self, distance: f32) {
        let mouse_pos = mouse_position();
        let delta_angle = {
            let dx = mouse_pos.0 - self.x;
            let dy = -(mouse_pos.1 - self.y);
            let mut target_angle = f32::atan2(dy,dx).to_degrees();
            match target_angle < 0.0 {
                true => target_angle = 180.0 + (180.0 + target_angle),
                false => (),
            }
            if target_angle == 360.0 {
                target_angle = 0.0;
            }

        };
    }
}