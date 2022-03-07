use log::info;

use babycat::Waveform;
use babycat::WaveformArgs;
use babycat::DECODING_BACKEND_SYMPHONIA;
use babycat::RESAMPLE_MODE_BABYCAT_LANCZOS;
use babycat::RESAMPLE_MODE_BABYCAT_SINC;
use babycat::RESAMPLE_MODE_LIBSAMPLERATE;

use crate::common::exit_with_msg;
use crate::common::UnwrapOrExit;

#[allow(clippy::too_many_arguments)]
pub fn convert(
    input_filename: &str,
    output_filename: &str,
    output_format: &str,
    start_time_milliseconds: usize,
    end_time_milliseconds: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    convert_to_mono: bool,
    zero_pad_ending: bool,
    resample_mode: &str,
    decoding_backend: &str,
) {
    //
    // Input validation.
    if output_format != "wav" {
        exit_with_msg(&format!(
            "Unsupported output file format: {}",
            output_format
        ));
    }
    let resample_mode_int = {
        if resample_mode == "libsamplerate" {
            RESAMPLE_MODE_LIBSAMPLERATE
        } else if resample_mode == "babycat_lanczos" {
            RESAMPLE_MODE_BABYCAT_LANCZOS
        } else if resample_mode == "babycat_sinc" {
            RESAMPLE_MODE_BABYCAT_SINC
        } else {
            panic!("NO");
        }
    };
    let decoding_backend_int = {
        if decoding_backend == "symphonia" {
            DECODING_BACKEND_SYMPHONIA
        } else {
            panic!("NO");
        }
    };
    //
    // Set up decoding.
    let waveform_args = WaveformArgs {
        start_time_milliseconds,
        end_time_milliseconds,
        frame_rate_hz,
        num_channels,
        convert_to_mono,
        zero_pad_ending,
        resample_mode: resample_mode_int,
        decoding_backend: decoding_backend_int,
    };
    //
    // Decode from filesystem.
    let decoding_start_time = std::time::Instant::now();
    let waveform = Waveform::from_file(input_filename, waveform_args).unwrap_or_exit();
    let decoding_elapsed = std::time::Instant::now() - decoding_start_time;
    info!(
        "Decoded {} frames of {} channels at {} hz in {} seconds from {}",
        waveform.num_frames(),
        waveform.num_channels(),
        waveform.frame_rate_hz(),
        decoding_elapsed.as_secs_f64(),
        input_filename,
    );
    //
    // Waveform is now in memory. Time to encode.
    let encoding_start_time = std::time::Instant::now();
    waveform.to_wav_file(output_filename).unwrap_or_exit();
    let encoding_elapsed = std::time::Instant::now() - encoding_start_time;
    info!(
        "Encoded as {} and saved in {} seconds to {}",
        output_format,
        encoding_elapsed.as_secs_f64(),
        output_filename,
    );
}
