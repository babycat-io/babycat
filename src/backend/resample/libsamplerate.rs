//! This module is not really part of Babycat's pubic API, but is made public
//! to make benchmarking Babycat internals easier.
//!
//! If you want to use Babycat to resample audio, you should decode
//! the audio into a [`Waveform`][crate::Waveform]
//! and then use the [`Waveform.resample()`][crate::Waveform#method.resample] method.

use crate::backend::errors::Error;
#[cfg(feature = "enable-libsamplerate")]
use crate::backend::resample::common::validate_args;

/// Resample input audio from one sample rate to another.
///
/// # Feature flags
/// This function is only available if the Cargo feature `enable-libsamplerate`
/// flag is enabled. The `enable-libsamplerate` flag is enabled by default
/// for the Babycat's Rust, Python, and C frontends, but is disabled
/// for the WebAssembly frontend.
///
#[allow(unused_variables)]
#[inline]
pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
) -> Result<Vec<f32>, Error> {
    #[cfg(feature = "enable-libsamplerate")]
    {
        validate_args(input_frame_rate_hz, output_frame_rate_hz, num_channels)?;

        match samplerate::convert(
            input_frame_rate_hz as u32,
            output_frame_rate_hz as u32,
            num_channels as usize,
            samplerate::converter_type::ConverterType::SincBestQuality,
            input_audio,
        ) {
            Ok(resampled) => Ok(resampled),
            Err(err) => {
                let samplerate::error::Error { .. } = err;
                match err.code() {
                    samplerate::error::ErrorCode::BadSrcRatio => Err(Error::WrongFrameRate(
                        input_frame_rate_hz,
                        output_frame_rate_hz,
                    )),
                    _ => Err(Error::ResamplingErrorWithMessage(
                        leak_str!(err.to_string()),
                    )),
                }
            }
        }
    }
    #[cfg(not(feature = "enable-libsamplerate"))]
    {
        Err(Error::FeatureNotCompiled("enable-libsamplerate"))
    }
}
