static FFMPEG_INIT: std::sync::Once = std::sync::Once::new();

#[inline(always)]
pub fn ffmpeg_init() {
    FFMPEG_INIT.call_once(|| {
        ffmpeg::init().unwrap();
        ffmpeg::util::log::set_level(ffmpeg::util::log::level::Level::Quiet);
    });
}

pub mod decoder;
pub mod decoder_iter;
pub mod frame_iter;
pub mod sample;
