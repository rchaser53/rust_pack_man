use std::path::Path;
use sdl2::mixer::{
    open_audio, allocate_channels,
    Music, Chunk,
    DEFAULT_CHANNELS, AUDIO_S16LSB
};

pub fn setup_sdl2_mixier(permit_channels_number: i32) -> () {
    let frequency = 44100;
    let format = AUDIO_S16LSB;          // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS;    // Stereo
    let chunk_size = 1024;

    open_audio(frequency, format, channels, chunk_size).unwrap();
    allocate_channels(permit_channels_number);
}

pub fn play_bgm(path: &Path) -> Music {
    return Music::from_file(path).unwrap();
}

pub fn play_sound_effect(path: &Path) -> Chunk {
    return Chunk::from_file(path).unwrap();
}