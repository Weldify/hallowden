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

macro_rules! load_sound {
    ($filepath:literal) => {
        load_sound_from_bytes(include_bytes!($filepath))
            .await
            .unwrap()
    }
}

macro_rules! load_texture {
    ($filepath:literal, $format:expr) => {
        Texture2D::from_file_with_format(include_bytes!($filepath), Some($format))
    }
}

impl Assets {
    pub async fn new() -> Assets {
        let spritesheet = load_texture!("../assets/spritesheet.png", ImageFormat::Png);
        spritesheet.set_filter(FilterMode::Nearest);

        let bounce_sound = load_sound!("../assets/bounce.ogg");
        let jump_sound = load_sound!("../assets/jump.ogg");
        let land_sound = load_sound!("../assets/land.ogg");
        let claim_sound = load_sound!("../assets/claim.ogg");

        Assets {
            spritesheet,
            bounce_sound,
            jump_sound,
            land_sound,
            claim_sound,
        }
    }
}
