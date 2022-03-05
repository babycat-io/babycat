//! This module is not really part of Babycat's pubic API, but is made public
//! to make benchmarking Babycat internals easier.
//!
//! If you want to use Babycat to resample audio, you should decode
//! the audio into a [`Waveform`][crate::Waveform]
//! and then use the [`Waveform.resample()`][crate::Waveform#method.resample] method.

use crate::backend::{
    errors::Error,
    resample::common::{get_num_output_frames, validate_args},
};

#[allow(clippy::excessive_precision)]
const KAISER_BEST_WINDOW: [f32; 32769] = include!("kaiser_best.txt");

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
    input_audio: &[f32],
) -> Result<Vec<f32>, Error> {
    validate_args(input_frame_rate_hz, output_frame_rate_hz, num_channels)?;

    let sample_ratio: f32 = (output_frame_rate_hz as f32) / (input_frame_rate_hz as f32);

    let num_output_frames = get_num_output_frames(
        input_audio,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
    );
    let ret_size = num_output_frames * (num_channels as usize);
    let mut ret: Vec<f32> = vec![0.0_f32; ret_size];

    let mut interp_win = KAISER_BEST_WINDOW;
    let precision = 512;
    if sample_ratio < 1.0 {
        for i in interp_win.iter_mut() {
            *i *= sample_ratio;
        }
    }

    // Same length as interp_win
    let interp_delta = interp_win
        .windows(2)
        .map(|w| w[1] - w[0])
        .chain(Some(0.0))
        .collect::<Vec<_>>();

    resample_f(
        input_audio,
        &mut ret,
        num_channels as usize,
        (output_frame_rate_hz as f64) / (input_frame_rate_hz as f64),
        &interp_win,
        &interp_delta,
        precision,
    );

    Ok(ret)
}

fn resample_f(
    in_audio: &[f32],
    out_audio: &mut [f32],
    num_channels: usize,
    sample_ratio: f64,
    interp_win: &[f32],
    interp_delta: &[f32],
    num_table: i32,
) {
    let scale = std::primitive::f64::min(sample_ratio, 1.0);

    // equal to (in_audio_hz / out_audio_hz)
    let time_increment = 1.0 / sample_ratio;
    let index_step = (scale * num_table as f64) as usize;

    let n_win = interp_win.len();
    let n_in_frames = in_audio.len() / (num_channels);
    let n_out_frames = out_audio.len() / (num_channels);

    for out_frame_idx in 0..n_out_frames {
        let time_register = time_increment * out_frame_idx as f64;
        let in_frame_idx = time_register as usize;

        let frac: f64 = scale * time_register.fract();
        let index_frac = frac * num_table as f64;
        let offset = index_frac as usize;
        let eta = index_frac.fract() as f32;

        let i_max = std::cmp::min(in_frame_idx + 1, (n_win - offset) / index_step);
        for i in 0..i_max {
            let weight =
                interp_win[offset + i * index_step] + eta * interp_delta[offset + i * index_step];
            for channel_idx in 0..num_channels {
                let out_idx: usize = (out_frame_idx * num_channels) + channel_idx;
                let in_idx: usize = ((in_frame_idx - i) * num_channels) + channel_idx;
                out_audio[out_idx] += weight * in_audio[in_idx];
            }
        }

        // Right wing of response
        let frac = scale - frac;
        let index_frac = frac * num_table as f64;
        let offset = index_frac as usize;
        let eta = index_frac.fract() as f32;

        let k_max = std::cmp::min(
            n_in_frames - in_frame_idx - 1,
            (n_win - offset) / index_step,
        );
        for k in 0..k_max {
            let weight =
                interp_win[offset + k * index_step] + eta * interp_delta[offset + k * index_step];
            for channel_idx in 0..num_channels {
                let out_idx: usize = (out_frame_idx * num_channels) + channel_idx;
                let in_idx: usize = ((in_frame_idx + k + 1) * num_channels) + channel_idx;
                out_audio[out_idx] += weight * in_audio[in_idx];
            }
        }
    }
}
