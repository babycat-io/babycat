use std::f32::consts::PI;

use babycat::resample::babycat_lanczos::resample as lanczos_resample;
use babycat::resample::babycat_sinc::resample as sinc_resample;
use babycat::resample::libsamplerate::resample as libsamplerate_resample;
use babycat::Waveform;

type Resampler = fn(u32, u32, u16, &[f32]) -> Result<Vec<f32>, babycat::Error>;

fn make_sine_wave(frequency: f32, frame_rate_hz: u32, duration: u32) -> Vec<f32> {
    (0..frame_rate_hz as usize * duration as usize)
        .map(|i| 2.0 * PI * frequency / frame_rate_hz as f32 * i as f32)
        .collect()
}

fn root_mean_square_of_diffs(v1: &[f32], v2: &[f32]) -> f64 {
    (((0..v1.len())
    // square each difference
    .map(|i| (v1[i] - v2[i]).powi(2))
    // sum the squared diffs
    .fold(0_f64, |acc, x| acc + x as f64))
    // divide by the number of differences
    / (v1.len() as f64))
        // square root
        .sqrt()
}

fn resample_down_and_up(
    resampler: Resampler,
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
    input_data: &[f32],
) -> Vec<f32> {
    let middle = resampler(
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        input_data,
    )
    .unwrap();
    let output = resampler(
        output_frame_rate_hz,
        input_frame_rate_hz,
        num_channels,
        &middle,
    )
    .unwrap();
    output
}

fn benchmark_func(
    resampler: Resampler,
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
    input: &[f32],
) -> (f64, f64) {
    let start_time = std::time::Instant::now();
    let output = &resample_down_and_up(
        resampler,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    let duration_microseconds = (std::time::Instant::now() - start_time).as_micros() as f64;
    let rms = root_mean_square_of_diffs(input, output);
    (rms, duration_microseconds)
}

fn benchmark_all_funcs(
    test_name: &str,
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u16,
    input: &[f32],
) {
    // libsamplerate is the "reference" sampler that we compare
    // all of the other resamplers to. That is because libsamplerate
    // should be the best because of all of the science and engineering
    // behind it.
    let (libsamplerate_rms, libsamplerate_microseconds) = benchmark_func(
        libsamplerate_resample,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    let (lanczos_rms, lanczos_microseconds) = benchmark_func(
        lanczos_resample,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    let (sinc_rms, sinc_microseconds) = benchmark_func(
        sinc_resample,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    println!(
        "{}: LIBSAMPLERATE is {}x more accurate than BABYCAT_LANCZOS ({}, {})",
        test_name,
        lanczos_rms / libsamplerate_rms,
        lanczos_rms,
        libsamplerate_rms
    );
    println!(
        "{}: LIBSAMPLERATE is {}x faster than BABYCAT_LANCZOS ({} us, {} us)\n",
        test_name,
        lanczos_microseconds / libsamplerate_microseconds,
        lanczos_microseconds,
        libsamplerate_microseconds
    );
    println!(
        "{}: LIBSAMPLERATE is {}x more accurate than BABYCAT_SINC ({}, {})",
        test_name,
        sinc_rms / libsamplerate_rms,
        sinc_rms,
        libsamplerate_rms
    );
    println!(
        "{}: LIBSAMPLERATE is {}x faster than BABYCAT_SINC ({} us, {} us)\n",
        test_name,
        sinc_microseconds / libsamplerate_microseconds,
        sinc_microseconds,
        libsamplerate_microseconds
    );
}

/// Benchmarks the resample using a very small single-channel waveform.
fn benchmark_small_vector() {
    let small_vector: Vec<f32> = vec![-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
    benchmark_all_funcs("small_vector_1", 4, 8, 1, &small_vector);
}

/// Benchmarks resampling a waveform representing a pure sine wave.
fn benchmark_sine_wave() {
    let sine_wave: Vec<f32> = make_sine_wave(512.0, 44100, 10);
    benchmark_all_funcs("sine_wave_1", 44100, 22050, 1, &sine_wave);
    benchmark_all_funcs("sine_wave_2", 44100, 44099, 1, &sine_wave);
    benchmark_all_funcs("sine_wave_3", 44100, 44101, 1, &sine_wave);
    benchmark_all_funcs("sine_wave_4", 44100, 88200, 1, &sine_wave);
}

/// Benchmarks a two-channel waveform that only plays sounds in the "left" channel.
///
/// The purpose of this benchmark is to verify that the resampler handles resampling
/// multiple channels separately.
fn benchmark_left_channel_tone() {
    let left_channel_tone: Vec<f32> = Waveform::from_file(
        "audio-for-tests/left-channel-tone/track.flac",
        Default::default(),
    )
    .unwrap()
    .to_interleaved_samples()
    .to_owned();
    benchmark_all_funcs("left_channel_tone_1", 44100, 4410, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_2", 44100, 11025, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_3", 44100, 22050, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_4", 44100, 44099, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_5", 44100, 48000, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_6", 44100, 88200, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_7", 44100, 96000, 2, &left_channel_tone);
    benchmark_all_funcs("left_channel_tone_8", 44100, 22050, 2, &left_channel_tone);
}

/// Benchmarks resampling the "blippy_trance" song.
fn benchmark_blippy_trance() {
    let blippy_trance: Vec<f32> = Waveform::from_file(
        "audio-for-tests/blippy-trance/track.wav",
        Default::default(),
    )
    .unwrap()
    .to_interleaved_samples()
    .to_owned();
    benchmark_all_funcs("blippy_trance_1", 44100, 4410, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_2", 44100, 11025, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_3", 44100, 22050, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_4", 44100, 44099, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_5", 44100, 48000, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_6", 44100, 88200, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_7", 44100, 96000, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_8", 44100, 22050, 2, &blippy_trance);
}

/// Benchmarks resampling the "on_hold_for_you" song.
fn benchmark_on_hold_for_you() {
    let on_hold_for_you: Vec<f32> = Waveform::from_file(
        "audio-for-tests/on-hold-for-you/track.flac",
        Default::default(),
    )
    .unwrap()
    .to_interleaved_samples()
    .to_owned();
    benchmark_all_funcs("on_hold_for_you_1", 44100, 4410, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_2", 44100, 11025, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_3", 44100, 22050, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_4", 44100, 44099, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_5", 44100, 48000, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_6", 44100, 88200, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_7", 44100, 96000, 2, &on_hold_for_you);
    benchmark_all_funcs("on_hold_for_you_8", 44100, 22050, 2, &on_hold_for_you);
}

fn main() {
    benchmark_small_vector();
    benchmark_sine_wave();
    benchmark_left_channel_tone();
    benchmark_blippy_trance();
    benchmark_on_hold_for_you();
}
