use crate::errors::Error;

pub fn get<T: Copy>(v: &[T], frame: usize, channel_idx: usize, num_channels: usize) -> T {
    v[frame * num_channels + channel_idx]
}

pub fn validate_args(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
) -> Result<(), Error> {
    if input_frame_rate_hz == 0 {
        return Err(Error::WrongInputFrameRate(input_frame_rate_hz));
    }
    if output_frame_rate_hz == 0 {
        return Err(Error::WrongOutputFrameRate(output_frame_rate_hz));
    }
    if num_channels == 0 {
        return Err(Error::WrongNumChannels(num_channels));
    }
    if (input_frame_rate_hz > output_frame_rate_hz)
        && (input_frame_rate_hz as f64 / output_frame_rate_hz as f64 > 256.0)
    {
        return Err(Error::WrongFrameRateRatio(
            input_frame_rate_hz,
            output_frame_rate_hz,
        ));
    }
    if output_frame_rate_hz as f64 / input_frame_rate_hz as f64 > 256.0 {
        return Err(Error::WrongFrameRateRatio(
            input_frame_rate_hz,
            output_frame_rate_hz,
        ));
    }
    Ok(())
}

pub fn get_num_output_frames(
    input_audio: &[f32],
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
) -> usize {
    ((input_audio.len() as f64 * output_frame_rate_hz as f64 / input_frame_rate_hz as f64).ceil()
        / num_channels as f64)
        .ceil() as usize
}
