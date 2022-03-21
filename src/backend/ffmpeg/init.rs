use std::sync::Once;

use ffmpeg_next::util::log::level::Level::Quiet;

static FFMPEG_INIT: Once = Once::new();

#[inline(always)]
pub fn ffmpeg_init() {
    FFMPEG_INIT.call_once(|| {
        ffmpeg_next::init().unwrap();
        ffmpeg_next::util::log::set_level(Quiet);
    });
}
