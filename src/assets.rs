use macroquad::{
    audio::{load_sound_from_bytes, set_sound_volume, Sound},
    prelude::ImageFormat,
    texture::{FilterMode, Texture2D},
};

pub struct Assets {
    pub spritesheet: Texture2D,
    pub bounce_sound: Sound,
    pub jump_sound: Sound,
    pub land_sound: Sound,
    pub claim_sound: Sound,
}

impl Assets {
    pub async fn new() -> Assets {
        let spritesheet_bytes = include_bytes!("../assets/spritesheet.png");
        let spritesheet =
            Texture2D::from_file_with_format(spritesheet_bytes, Some(ImageFormat::Png));
        spritesheet.set_filter(FilterMode::Nearest);

        let bounce_sound = load_sound_from_bytes(include_bytes!("../assets/bounce.ogg"))
            .await
            .unwrap();
        let jump_sound = load_sound_from_bytes(include_bytes!("../assets/jump.ogg"))
            .await
            .unwrap();
        let land_sound = load_sound_from_bytes(include_bytes!("../assets/land.ogg"))
            .await
            .unwrap();
        let claim_sound = load_sound_from_bytes(include_bytes!("../assets/claim.ogg"))
            .await
            .unwrap();

        Assets {
            spritesheet,
            bounce_sound,
            jump_sound,
            land_sound,
            claim_sound,
        }
    }
}
