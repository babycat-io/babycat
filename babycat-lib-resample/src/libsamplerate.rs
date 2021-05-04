use crate::errors::Error;

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
) -> Result<Vec<f32>, Error> {
    crate::common::validate_args(input_frame_rate_hz, output_frame_rate_hz, num_channels)?;

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
                _ => Err(Error::UnknownError(Box::leak(
                    err.to_string().into_boxed_str(),
                ))),
            }
        }
    }
}
