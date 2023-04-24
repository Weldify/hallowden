use macroquad::audio::{Sound, play_sound, PlaySoundParams};

pub fn play_sound_once_vol(sound: Sound, volume: f32) {
    play_sound(sound, PlaySoundParams {
        looped: false,
        volume,
    })
}