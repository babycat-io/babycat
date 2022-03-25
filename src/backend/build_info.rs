//! Information about compile-time features and licensing.

/// Returns `true` if Babycat was compiled with support for
/// reading and writing files from/to the local filesystem.
#[inline]
pub fn compiled_with_filesystem() -> bool {
    cfg!(feature = "enable-filesystem")
}

/// Returns `true` if Babycat was compiled with multithreading support.
#[inline]
pub fn compiled_with_multithreading() -> bool {
    cfg!(feature = "enable-multithreading")
}

/// Returns `true` if Babycat was compiled with FFmpeg support enabled.
///
/// This function will return `true` no matter how
/// FFmpeg was compiled or linked to Babycat.
#[inline]
pub fn compiled_with_ffmpeg() -> bool {
    cfg!(feature = "enable-ffmpeg")
}

/// Returns `true` if Babycat was statically linked to an existing copy of FFmpeg.
#[inline]
pub fn compiled_with_ffmpeg_link_static() -> bool {
    cfg!(feature = "enable-ffmpeg-link-static")
}

/// Returns `true` if Babycat compiled its own copy of FFmpeg.
#[inline]
pub fn compiled_with_ffmpeg_build_link_static() -> bool {
    cfg!(feature = "enable-ffmpeg-build-link-static")
}

const MIT_LICENSE: &str = "MIT";
const LGPL_2_1_OR_LATER_LICENSE: &str = "LGPL-2.1+";

/// The copyright license for this version of Babycat.
///
/// This could change based on which features or libraries
/// were compiled into Babycat.
#[inline]
pub fn copyright_license_spdx() -> &'static str {
    if cfg!(feature = "enable-ffmpeg") {
        return LGPL_2_1_OR_LATER_LICENSE;
    }
    MIT_LICENSE
}

/// The current Babycat version.
///
/// This function returns `"0.0.0"` for development versions
/// of Babycat.
#[inline]
pub fn babycat_version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0")
}
