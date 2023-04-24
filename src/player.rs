use macroquad::{
    audio::{play_sound, play_sound_once, PlaySoundParams, set_sound_volume},
    prelude::*,
};

use crate::{actor::Actor, assets::Assets, solid::Solid, utils::play_sound_once_vol, lantern::Lantern};

pub struct Player {
    velocity: Vec2,
    is_flipped: bool,
    is_grounded: bool,
    jump_power: f32,

    is_dead: bool,
    pub respawn_position: Vec2,

    pub actor: Actor,
}

impl Player {
    pub fn new() -> Player {
        Player {
            velocity: Vec2::ZERO,
            is_flipped: false,
            is_grounded: false,
            jump_power: 0.0,

            is_dead: false,
            respawn_position: Vec2::ZERO,

            actor: Actor::new(Rect::new(0.0, 0.0, 6.0, 8.0)),
        }
    }

    fn move_ground(&mut self) {
        // move_ground has no authority over aerial movement
        if !self.is_grounded {
            return;
        }

        let ground_move =
            is_key_down(KeyCode::D) as i32 as f32 - is_key_down(KeyCode::A) as i32 as f32;

        // No frame time since this is non accumulating
        self.velocity.x = ground_move * 15.0;
    }

    fn do_jump(&mut self, assets: &Assets) {
        if !self.is_grounded {
            self.jump_power = 0.0;
            return;
        }

        // Charging
        if is_key_down(KeyCode::W) {
            self.velocity.x = 0.0;
            self.jump_power = (self.jump_power.max(0.3) + get_frame_time()).min(1.0);
            return;
        }

        if self.jump_power == 0.0 {
            return;
        }

        self.velocity.x = match self.is_flipped {
            true => -1.0,
            false => 1.0,
        } * self.jump_power
            * 50.0;

        self.velocity.y = -self.jump_power * 100.0;
        self.jump_power = 0.0;
        self.is_grounded = false;

		play_sound_once_vol(assets.jump_sound, 0.5);
    }

    pub fn can_claim_lantern(&self, lantern: &Lantern) -> bool {
        self.actor.collider.point().distance(lantern.pos) < 10.0
    }

    pub fn claim_lantern(&mut self, lantern: &mut Lantern, assets: &Assets) {
        lantern.is_claimed = true;
        self.respawn_position = lantern.pos;

        play_sound_once_vol(assets.claim_sound, 0.5);
    }

    fn fly_towards_spawnpoint(&mut self) {
        let fly_dir = (self.respawn_position - self.actor.collider.point()).normalize_or_zero();
        self.actor.collider = self.actor.collider.offset(fly_dir * get_frame_time() * 100.0);

        // Still flying to it
        if self.actor.collider.point().distance(self.respawn_position) > 2.0 {
            return;
        }

        self.actor.collider.move_to(self.respawn_position);
        self.is_dead = false;
    }

    pub fn update(&mut self, assets: &Assets, solids: &Vec<Solid>) {
        if self.is_dead {
            self.fly_towards_spawnpoint();
            return;
        }

        self.move_ground();

        if self.velocity.x != 0.0 {
            self.is_flipped = self.velocity.x < 0.0;
        }

        self.do_jump(assets);

        self.velocity.y += get_frame_time() * 98.0;

        let x_collision = self
            .actor
            .move_x(self.velocity.x * get_frame_time(), solids);
        let y_collision = self
            .actor
            .move_y(self.velocity.y * get_frame_time(), solids);

        // Bounce
        if !self.is_grounded && x_collision.has_collided {
            self.velocity.x = -self.velocity.x;
			self.is_flipped = self.velocity.x < 0.0;
			play_sound_once_vol(assets.bounce_sound, 0.35);
        }

        // Only update grounded if we actually moved
        if y_collision.has_moved {
			let was_grounded = self.is_grounded;
            self.is_grounded = y_collision.has_collided && y_collision.collision_side > 0.0;

			if !was_grounded && self.is_grounded {
				play_sound_once_vol(assets.land_sound, 0.5);
			}
        }

        if y_collision.has_collided {
            // Splat and DIE!
            if self.velocity.y > 100.0 {
                self.is_dead = true;
            }

            self.velocity.y = 0.0;
        }
    }

    pub fn draw_soul(&self) {
        let center = self.actor.collider.center();
        draw_circle(center.x, center.y, 3.0, WHITE);
    }

    pub fn draw(&self, assets: &Assets) {
        if self.is_dead {
            self.draw_soul();
            return;
        }
        
        let mut sprite_x: f32 = 0.0;

        // Charging jump sprite
        if self.jump_power > 0.0 {
            sprite_x = 8.0;
        }

        if !self.is_grounded {
            sprite_x = match self.velocity.y < 0.0 {
                true => 16.0,  // Jump
                false => 24.0, // Fall
            }
        }

        draw_texture_ex(
            assets.spritesheet,
            self.actor.collider.x - 1.0, // Collider is 6 wide, sprite is 8 wide
            self.actor.collider.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(sprite_x, 0.0, 8.0, 8.0)),
                flip_x: self.is_flipped,
                ..Default::default()
            },
        );
    }
}
