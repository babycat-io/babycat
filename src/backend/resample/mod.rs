pub mod babycat;
pub mod common;
pub mod lanczos;
pub mod libsamplerate;

use crate::backend::decode_args::BABYCAT_DEFAULT_RESAMPLE_MODE;
use crate::backend::decode_args::BABYCAT_RESAMPLE_MODE_BABYCAT;
use crate::backend::decode_args::BABYCAT_RESAMPLE_MODE_LANCZOS;
use crate::backend::decode_args::BABYCAT_RESAMPLE_MODE_LIBSAMPLERATE;
use crate::backend::errors::Error;

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
    resample_mode: u32,
) -> Result<Vec<f32>, Error> {
    match resample_mode {
        BABYCAT_DEFAULT_RESAMPLE_MODE => libsamplerate::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        BABYCAT_RESAMPLE_MODE_LIBSAMPLERATE => libsamplerate::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        BABYCAT_RESAMPLE_MODE_LANCZOS => lanczos::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        BABYCAT_RESAMPLE_MODE_BABYCAT => babycat::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),
        _ => Err(Error::FeatureNotCompiled("resample")),
    }
}
