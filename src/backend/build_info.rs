#[inline(always)]
pub fn compiled_with_filesystem() -> bool {
    cfg!(feature = "enable-filesystem")
}

#[inline(always)]
pub fn compiled_with_multithreading() -> bool {
    cfg!(feature = "enable-multithreading")
}

#[inline(always)]
pub fn compiled_with_ffmpeg() -> bool {
    cfg!(feature = "enable-ffmpeg")
}

#[inline(always)]
pub fn compiled_with_ffmpeg_link_static() -> bool {
    cfg!(feature = "enable-ffmpeg-link-static")
}

#[inline(always)]
pub fn compiled_with_ffmpeg_build_link_static() -> bool {
    cfg!(feature = "enable-ffmpeg-build-link-static")
}

const MIT_LICENSE: &str = "MIT";
const LGPL_2_1_OR_LATER_LICENSE: &str = "LGPL-2.1+";

#[inline(always)]
pub fn copyright_license_spdx() -> &'static str {
    if cfg!(feature = "enable-ffmpeg") {
        return LGPL_2_1_OR_LATER_LICENSE;
    }
    MIT_LICENSE
}

#[inline(always)]
pub fn babycat_version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0")
}
