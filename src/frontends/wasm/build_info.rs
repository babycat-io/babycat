#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

/// Returns ``true`` if Babycat was compiled with support for
/// reading and writing files from/to the local filesystem.
#[allow(clippy::unused_unit)]
#[wasm_bindgen]
pub struct BuildInfo {}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl BuildInfo {
    /// Returns ``true`` if Babycat was compiled with multithreading support.
    pub fn compiledWithFilesystem() -> bool {
        crate::backend::build_info::compiled_with_filesystem()
    }

    /// Returns ``true`` if Babycat was compiled with multithreading support.
    pub fn compiledWithMultithreading() -> bool {
        crate::backend::build_info::compiled_with_multithreading()
    }

    /// Returns ``true`` if Babycat was compiled with FFmpeg support enabled.
    ///
    /// This function will return ``true`` no matter how
    /// FFmpeg was compiled or linked to Babycat.
    pub fn compiledWithFFmpeg() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg()
    }

    /// Returns ``true`` if Babycat was statically linked to an existing copy of FFmpeg.
    pub fn compiledWithFFmpegLinkStatic() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg_link_static()
    }

    /// Returns ``true`` if Babycat compiled its own copy of FFmpeg.
    pub fn compiledWithFFpegBuildLinkStatic() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg_build_link_static()
    }

    /// The copyright license for this version of Babycat.
    ///
    /// This could change based on which features or libraries
    /// were compiled into Babycat.
    pub fn copyrightLicenseSPDX() -> String {
        crate::backend::build_info::copyright_license_spdx().into()
    }

    /// The current Babycat version.
    ///
    /// This function returns ``"0.0.0"`` for development versions
    /// of Babycat.
    pub fn babycatVersion() -> String {
        crate::backend::build_info::babycat_version().into()
    }
}
