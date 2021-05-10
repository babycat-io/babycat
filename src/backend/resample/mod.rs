//! This module is not really part of Babycat's pubic API, but is made public
//! to make benchmarking Babycat internals easier.
//!
//! If you want to use Babycat to resample audio, you should decode
//! the audio into a [`FloatWaveform`][crate::FloatWaveform]
//! and then use the [`FloatWaveform.resample()`][crate::FloatWaveform#method.resample] method.

pub mod babycat;
pub mod common;
pub mod lanczos;
pub mod libsamplerate;

use crate::backend::decode_args::DEFAULT_RESAMPLE_MODE;
use crate::backend::decode_args::RESAMPLE_MODE_BABYCAT;
use crate::backend::decode_args::RESAMPLE_MODE_LANCZOS;
use crate::backend::decode_args::RESAMPLE_MODE_LIBSAMPLERATE;
use crate::backend::errors::Error;

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
    resample_mode: u32,
) -> Result<Vec<f32>, Error> {
    match resample_mode {
        DEFAULT_RESAMPLE_MODE => {
            if cfg!(feature = "enable-libsamplerate") {
                libsamplerate::resample(
                    input_frame_rate_hz,
                    output_frame_rate_hz,
                    num_channels,
                    input_audio,
                )
            } else {
                lanczos::resample(
                    input_frame_rate_hz,
                    output_frame_rate_hz,
                    num_channels,
                    input_audio,
                )
            }
        }

        RESAMPLE_MODE_LIBSAMPLERATE => libsamplerate::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        RESAMPLE_MODE_LANCZOS => lanczos::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        RESAMPLE_MODE_BABYCAT => babycat::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),
        _ => Err(Error::FeatureNotCompiled("resample")),
    }
}
