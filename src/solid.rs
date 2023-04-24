use macroquad::{prelude::{Rect, YELLOW}, shapes::draw_rectangle_lines};

pub struct Solid {
    pub collider: Rect,
}

impl Solid {
    pub fn new(rect: Rect) -> Solid {
        Solid {
            collider: rect,
        }
    }

    pub fn draw_debug(&self) {
        draw_rectangle_lines(
            self.collider.x,
            self.collider.y,
            self.collider.w,
            self.collider.h,
            2.0,
            YELLOW,
        );
    }
}