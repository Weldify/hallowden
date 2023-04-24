use macroquad::{prelude::{Vec2, WHITE, Rect}, texture::{draw_texture_ex, DrawTextureParams}, shapes::draw_line};

use crate::{assets::Assets, utils::play_sound_once_vol};

pub struct Lantern {
    pub pos: Vec2,
    pub is_claimed: bool,
}

impl Lantern {
    pub fn new(pos: Vec2) -> Lantern {
        Lantern {
            pos,
            is_claimed: false,
        }
    }
    
    pub fn draw(&self, assets: &Assets) {
        draw_texture_ex(
            assets.spritesheet,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(32.0, 0.0, 8.0, 8.0)),
                ..Default::default()
            },
        );

        if !self.is_claimed {
            return;
        }

        draw_texture_ex(
            assets.spritesheet,
            self.pos.x,
            self.pos.y - 8.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(40.0, 0.0, 8.0, 8.0)),
                ..Default::default()
            },
        )
    }
}