//! This module is not really part of Babycat's pubic API, but is made public
//! to make benchmarking Babycat internals easier.
//!
//! If you want to use Babycat to resample audio, you should decode
//! the audio into a [`Waveform`][crate::Waveform]
//! and then use the [`Waveform.resample()`][crate::Waveform#method.resample] method.

#[doc(hidden)]
pub mod babycat_lanczos;
#[doc(hidden)]
pub mod babycat_sinc;
#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub mod libsamplerate;

use crate::backend::constants::DEFAULT_RESAMPLE_MODE;
use crate::backend::constants::RESAMPLE_MODE_BABYCAT_LANCZOS;
use crate::backend::constants::RESAMPLE_MODE_BABYCAT_SINC;
use crate::backend::constants::RESAMPLE_MODE_LIBSAMPLERATE;
use crate::backend::errors::Error;

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
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
                babycat_sinc::resample(
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

        RESAMPLE_MODE_BABYCAT_LANCZOS => babycat_lanczos::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),

        RESAMPLE_MODE_BABYCAT_SINC => babycat_sinc::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        ),
        _ => Err(Error::FeatureNotCompiled("resample")),
    }
}
