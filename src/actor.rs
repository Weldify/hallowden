use macroquad::{
    prelude::{vec2, Rect, Vec2, BLUE},
    shapes::draw_rectangle_lines,
};

use crate::solid::Solid;

pub struct ActorCollisionResult {
    pub collision_side: f32,
    pub has_collided: bool,
    pub has_moved: bool,
}

impl ActorCollisionResult {
    fn new(moved: bool, collided: bool, side: f32) -> ActorCollisionResult {
        ActorCollisionResult {
            collision_side: side,
            has_collided: collided,
            has_moved: moved,
        }
    }
}

pub struct Actor {
    pub collider: Rect,
    x_remainder: f32,
    y_remainder: f32,
}

impl Actor {
    pub fn new(rect: Rect) -> Actor {
        Actor {
            collider: rect,
            x_remainder: 0.0,
            y_remainder: 0.0,
        }
    }

    // Returns whether we collided with a solid
    pub fn move_x(&mut self, amount: f32, solids: &Vec<Solid>) -> ActorCollisionResult {
        self.x_remainder += amount;

        let mut movement = self.x_remainder.round();
        if movement == 0.0 {
            return ActorCollisionResult::new(false, false, 0.0);
        }

        self.x_remainder -= movement;
        let sign = movement.signum();

        while movement != 0.0 {
            if !self.is_colliding_solids(vec2(sign, 0.0), solids) {
                self.collider.x = (self.collider.x + sign).round();
                movement = (movement - sign).round();
            } else {
                return ActorCollisionResult::new(true, true, sign);
            }
        }

        ActorCollisionResult::new(true, false, 0.0)
    }

    pub fn move_y(&mut self, amount: f32, solids: &Vec<Solid>) -> ActorCollisionResult {
        self.y_remainder += amount;

        let mut movement = self.y_remainder.round();
        if movement == 0.0 {
            return ActorCollisionResult::new(false, false, 0.0);
        }

        self.y_remainder -= movement;
        let sign = movement.signum();

        while movement != 0.0 {
            if !self.is_colliding_solids(vec2(0.0, sign), solids) {
                self.collider.y = (self.collider.y + sign).round();
                movement = (movement - sign).round();
            } else {
                return ActorCollisionResult::new(true, true, sign);
            }
        }

        ActorCollisionResult::new(true, false, 0.0)
    }

    fn is_colliding_solids(&self, offset: Vec2, solids: &Vec<Solid>) -> bool {
        let rect = self.collider.offset(offset);
        solids.iter().any(|s| {
            // Can't use .overlaps(), we need to check for touch
            rect.left() < s.collider.right()
                && rect.right() > s.collider.left()
                && rect.top() < s.collider.bottom()
                && rect.bottom() > s.collider.top()
        })
    }

    pub fn draw_debug(&self) {
        draw_rectangle_lines(
            self.collider.x,
            self.collider.y,
            self.collider.w,
            self.collider.h,
            2.0,
            BLUE,
        );
    }
}
