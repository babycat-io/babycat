use crate::backend::errors::Error;
use crate::backend::resample::common::get;
use crate::backend::resample::common::get_num_output_frames;
use crate::backend::resample::common::validate_args;
use std::f32::consts::PI;

const KERNEL_A: i32 = 5;

fn lanczos_kernel(x: f32, a: f32) -> f32 {
    if float_cmp::approx_eq!(f32, x, 0.0_f32) {
        return 1.0;
    }
    if -a <= x && x < a {
        return (a * (PI * x).sin() * (PI * x / a).sin()) / (PI * PI * x * x);
    }
    0.0
}

pub fn compute_sample(
    input_audio: &[f32],
    frame_idx: f32,
    channel_idx: usize,
    num_channels: usize,
) -> f32 {
    let num_input_frames: u64 = input_audio.len() as u64 / num_channels as u64;
    let a: f32 = KERNEL_A as f32;
    let x_floor = frame_idx as i64;
    let i_start = x_floor - a as i64 + 1;
    let i_end = x_floor + a as i64 + 1;
    let mut the_sample: f32 = 0.0_f32;
    for i in i_start..i_end {
        if (i as u64) < num_input_frames {
            the_sample += get(input_audio, i as usize, channel_idx, num_channels)
                * lanczos_kernel(frame_idx - i as f32, a)
        }
    }
    the_sample
}

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
) -> Result<Vec<f32>, Error> {
    validate_args(input_frame_rate_hz, output_frame_rate_hz, num_channels)?;
    let output_num_frames = get_num_output_frames(
        input_audio,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
    );
    let output_audio: Vec<f32> = (0..output_num_frames)
        .flat_map(|input_frame_idx| {
            (0..num_channels as usize).map(move |channel_idx| {
                let output_frame_idx = (input_frame_idx as f32 * input_frame_rate_hz as f32)
                    / output_frame_rate_hz as f32;
                compute_sample(
                    input_audio,
                    output_frame_idx,
                    channel_idx,
                    num_channels as usize,
                )
            })
        })
        .collect();

    Ok(output_audio)
}
