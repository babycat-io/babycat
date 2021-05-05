use std::f32::consts::PI;

use babycat::resample::lanczos::resample as lanczos_resample;
use babycat::resample::libsamplerate::resample as libsamplerate_resample;
use babycat::FloatWaveform;
//use babycat::resample::babycat::resample as babycat_resample;

type Resampler = fn(u32, u32, u32, &[f32]) -> Result<Vec<f32>, babycat::Error>;

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
    num_channels: u32,
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
    num_channels: u32,
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
    num_channels: u32,
    input: &[f32],
) {
    let (lanczos_rms, lanczos_microseconds) = benchmark_func(
        lanczos_resample,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    let (libsamplerate_rms, libsamplerate_microseconds) = benchmark_func(
        libsamplerate_resample,
        input_frame_rate_hz,
        output_frame_rate_hz,
        num_channels,
        &input,
    );
    println!(
        "{}: libsamplerate is {}x more accurate ({}, {})",
        test_name,
        lanczos_rms / libsamplerate_rms,
        lanczos_rms,
        libsamplerate_rms
    );
    println!(
        "{}: libsamplerate is {}x faster ({} us, {} us)\n",
        test_name,
        lanczos_microseconds / libsamplerate_microseconds,
        lanczos_microseconds,
        libsamplerate_microseconds
    );
}

fn main() {
    let small_vector_1: Vec<f32> = vec![-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
    let sine_wave_1: Vec<f32> = make_sine_wave(512.0, 44100, 10);
    let blippy_trance: Vec<f32> = FloatWaveform::from_file(
        "audio-for-tests/blippy-trance/track.mp3",
        Default::default(),
    )
    .unwrap()
    .interleaved_samples()
    .to_owned();

    benchmark_all_funcs("small_vector_1", 4, 8, 1, &small_vector_1);
    benchmark_all_funcs("sine_wave_1", 44100, 22050, 1, &sine_wave_1);
    benchmark_all_funcs("blippy_trance_1", 44100, 4410, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_2", 44100, 11025, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_3", 44100, 22050, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_4", 44100, 44099, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_5", 44100, 48000, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_6", 44100, 88200, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_7", 44100, 96000, 2, &blippy_trance);
    benchmark_all_funcs("blippy_trance_8", 44100, 22050, 2, &blippy_trance);
}
