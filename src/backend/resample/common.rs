//! This module is not really part of Babycat's pubic API, but is made public
//! to make benchmarking Babycat internals easier.
//!
//! If you want to use Babycat to resample audio, you should decode
//! the audio into a [`Waveform`][crate::Waveform]
//! and then use the [`Waveform.resample()`][crate::Waveform#method.resample] method.

use crate::backend::errors::Error;

pub fn get<T: Copy>(v: &[T], frame: usize, channel_idx: usize, num_channels: usize) -> T {
    v[frame * num_channels + channel_idx]
}

pub fn validate_args(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
) -> Result<(), Error> {
    if input_frame_rate_hz == 0 || output_frame_rate_hz == 0 {
        return Err(Error::WrongFrameRate(
            input_frame_rate_hz,
            output_frame_rate_hz,
        ));
    }
    if num_channels == 0 {
        return Err(Error::ResamplingError);
    }
    if (input_frame_rate_hz > output_frame_rate_hz)
        && (f64::from(input_frame_rate_hz) / f64::from(output_frame_rate_hz) > 256.0)
    {
        return Err(Error::WrongFrameRateRatio(
            input_frame_rate_hz,
            output_frame_rate_hz,
        ));
    }
    if f64::from(output_frame_rate_hz) / f64::from(input_frame_rate_hz) > 256.0 {
        return Err(Error::WrongFrameRateRatio(
            input_frame_rate_hz,
            output_frame_rate_hz,
        ));
    }
    Ok(())
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
pub fn get_num_output_frames(
    input_audio: &[f32],
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
) -> usize {
    ((input_audio.len() as f64 * f64::from(output_frame_rate_hz) / f64::from(input_frame_rate_hz))
        .ceil()
        / f64::from(num_channels))
    .ceil() as usize
}
