use macroquad::prelude::*;

use crate::{
    assets::Assets, lantern::Lantern, player::Player, solid::Solid, utils::play_sound_once_vol,
};

pub struct Game {
    pub game_width: u32,
    pub game_height: u32,
    pub camera_pos: Vec2,

    assets: Assets,

    solids: Vec<Solid>,

    player: Player,
    lanterns: Vec<Lantern>,
}

const COL_MARKER: Color = color_u8!(255, 0, 0, 255);
const EMPTY_COLOR: Color = color_u8!(0, 0, 0, 0);

fn read_collision_rect(image: &mut Image, x: u32, y: u32) -> Rect {
    // 1. Find right bound of rect
    let right_bound_opt = (x + 1..image.width as u32)
        .into_iter()
        .find(|&pos_x| image.get_pixel(pos_x, y) == COL_MARKER);

    let Some(right_bound) = right_bound_opt else {
        panic!("Right bound of collision rect not found: leftx={x} topy={y}");
    };

    // 2. Find bottom right bound of rect
    let bottom_bound_opt = (y + 1..image.height as u32)
        .into_iter()
        .find(|&pos_y| image.get_pixel(x, pos_y) == COL_MARKER);

    let Some(bottom_bound) = bottom_bound_opt else {
        panic!("Bottom right bound of collision rect not found!: rightx={right_bound} topy={y}")
    };

    // Make sure this collision rect cant be read anymore
    image.set_pixel(x, y, EMPTY_COLOR);
    image.set_pixel(right_bound, y, EMPTY_COLOR);
    image.set_pixel(x, bottom_bound, EMPTY_COLOR);
    image.set_pixel(right_bound, bottom_bound, EMPTY_COLOR);

    Rect::new(
        x as f32,
        y as f32,
        (right_bound + 1 - x) as f32,
        (bottom_bound + 1 - y) as f32,
    )
}

fn read_map_collisions(mut image: Image) -> Vec<Solid> {
    let mut solids = vec![];

    for y_pos in 0..image.height {
        for x_pos in 0..image.width {
            let col = image.get_pixel(x_pos as u32, y_pos as u32);
            if col != COL_MARKER {
                continue;
            }

            let rect = read_collision_rect(&mut image, x_pos as u32, y_pos as u32);
            solids.push(Solid::new(rect));
        }
    }

    solids
}

impl Game {
    pub async fn new() -> Game {
        let assets = Assets::new().await;

        let mut solids = read_map_collisions(assets.map_collision_image.clone());

        // Map bounds
        solids.push(Solid::new(Rect::new(-1.0, 0.0, 1.0, 600.0)));
        solids.push(Solid::new(Rect::new(100.0, 0.0, 1.0, 600.0)));
        solids.push(Solid::new(Rect::new(0.0, -1.0, 100.0, 1.0)));
        solids.push(Solid::new(Rect::new(0.0, 600.0, 100.0, 1.0)));

        let mut player = Player::new();
        player.actor.collider.y = 8.0;

        let mut game = Game {
            game_width: 100,
            game_height: 150,
            camera_pos: vec2(50.0, 0.0),

            assets,

            player,
            solids,

            lanterns: vec![
                Lantern::new(vec2(7.0, 19.0)),
                Lantern::new(vec2(42.0, 90.0)),
                Lantern::new(vec2(1.0, 174.0)),
                Lantern::new(vec2(71.0, 262.0)),
            ],
        };

        let first_lantern = &mut game.lanterns[0];
        game.player.claim_lantern(first_lantern, &game.assets);

        game
    }

    fn claim_lanterns(&mut self) {
        let index_to_claim_opt = self
            .lanterns
            .iter()
            .enumerate()
            .find(|(i, l)| self.player.can_claim_lantern(l) && !l.is_claimed)
            .map(|pair| pair.0);

        let Some(index_to_claim) = index_to_claim_opt else {
            return;
        };

        self.lanterns.iter_mut().for_each(|l| l.is_claimed = false);
        self.player
            .claim_lantern(&mut self.lanterns[index_to_claim], &self.assets)
    }

    pub fn update(&mut self) {
        self.player.update(&self.assets, &self.solids);
        if !self.player.is_dead {
            self.claim_lanterns();
        }

        self.camera_pos.y = self.player.actor.collider.center().y;
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        self.solids.iter().for_each(|s| s.draw_debug());
        self.lanterns.iter().for_each(|l| l.draw(&self.assets));
        self.player.draw(&self.assets);
    }
}
