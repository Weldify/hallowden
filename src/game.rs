use macroquad::prelude::*;

use crate::{
    assets::Assets, lantern::Lantern, player::Player, solid::Solid, utils::play_sound_once_vol,
};

pub struct Game {
    pub game_width: u32,
    pub game_height: u32,

    assets: Assets,

    solids: Vec<Solid>,

    player: Player,
    lanterns: Vec<Lantern>,
}

impl Game {
    pub async fn new() -> Game {
        let mut game = Game {
            game_width: 100,
            game_height: 150,

            assets: Assets::new().await,

            player: Player::new(),
            solids: vec![
                Solid::new(Rect::new(0.0, 140.0, 50.0, 10.0)),
                Solid::new(Rect::new(50.0, 100.0, 30.0, 40.0)),
                Solid::new(Rect::new(20.0, 80.0, 5.0, 30.0)),
            ],

            lanterns: vec![
                Lantern::new(vec2(10.0, 132.0)),
                Lantern::new(vec2(21.0, 72.0)),
            ],
        };

        let first_lantern = &mut game.lanterns[0];
        game.player.claim_lantern(first_lantern, &game.assets);

        game
    }

    fn claim_lanterns(&mut self) {
        let lantern_to_claim = self
            .lanterns
            .iter()
            .filter(|l| self.player.can_claim_lantern(l) && !l.is_claimed)
            .next();

        if let Some(claimed_lantern) = lantern_to_claim {
            for lantern in self.lanterns.iter_mut() {
                if std::ptr::eq(lantern, claimed_lantern) {
                    self.player.claim_lantern(lantern, &self.assets);
                } else {
                    lantern.is_claimed = false;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.player.update(&self.assets, &self.solids);
        self.claim_lanterns()
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        self.solids.iter().for_each(|s| s.draw_debug());
        self.lanterns.iter().for_each(|l| l.draw(&self.assets));
        self.player.draw(&self.assets);
    }
}
