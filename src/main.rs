#![windows_subsystem = "windows"]

use macroquad::{prelude::*};

mod game;
mod assets;
mod utils;

mod actor;
mod solid;

mod player;
mod lantern;

use game::*;

#[macroquad::main("Hallowden")]
async fn main() {
    let mut game = Game::new().await;

    let render_targ = render_target(game.game_width, game.game_height);
    render_targ.texture.set_filter(FilterMode::Nearest);

    let mut render_camera = Camera2D::from_display_rect(Rect::new(
        0.0,
        0.0,
        game.game_width as f32,
        game.game_height as f32,
    ));
    render_camera.render_target = Some(render_targ);

    loop {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let game_size = Vec2::new(game.game_width as f32, game.game_height as f32);

        let hor_scale = (screen_size.x / game_size.x).floor().max(1.0);
        let ver_scale = (screen_size.y / game_size.y).floor().max(1.0);
        let render_scale = hor_scale.min(ver_scale);
        let render_size = game_size * render_scale;

        let render_params = DrawTextureParams {
            dest_size: Some(render_size),
            flip_x: false,
            flip_y: true,

            ..Default::default()
        };

        game.update();

        set_camera(&render_camera);
        game.draw();
        set_default_camera();

        draw_texture_ex(
            render_targ.texture,
            (screen_size.x / 2.0 - render_size.x / 2.0).round(),
            (screen_size.y / 2.0 - render_size.y / 2.0).round(),
            WHITE,
            render_params,
        );

        next_frame().await
    }
}
