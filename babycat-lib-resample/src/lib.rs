pub mod errors;

pub mod common;

#[cfg(feature = "lanczos")]
pub mod lanczos;

#[cfg(feature = "libsamplerate")]
pub mod libsamplerate;

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
) -> Result<Vec<f32>, errors::Error> {
    #[cfg(all(feature = "lanczos"))]
    {
        lanczos::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        )
    }

    #[cfg(all(feature = "libsamplerate", not(feature = "lanczos")))]
    {
        libsamplerate::resample(
            input_frame_rate_hz,
            output_frame_rate_hz,
            num_channels,
            input_audio,
        )
    }
    #[cfg(not(any(feature = "lanczos", feature = "libsamplerate")))]
    {
        Err(errors::Error::FeatureNotCompiled)
    }
}
