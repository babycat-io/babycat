#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
pub struct BuildInfo {}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl BuildInfo {
    pub fn compiledWithFilesystem() -> bool {
        crate::backend::build_info::compiled_with_filesystem()
    }

    pub fn compiledWithMultithreading() -> bool {
        crate::backend::build_info::compiled_with_multithreading()
    }

    pub fn compiledWithFFmpeg() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg()
    }

    pub fn compiledWithFFmpegLinkStatic() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg_link_static()
    }

    pub fn compiledWithFFpegBuildLinkStatic() -> bool {
        crate::backend::build_info::compiled_with_ffmpeg_build_link_static()
    }

    pub fn copyrightLicenseSPDX() -> String {
        crate::backend::build_info::copyright_license_spdx().into()
    }

    pub fn babycatVersion() -> String {
        crate::backend::build_info::babycat_version().into()
    }
}
