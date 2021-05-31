use crate::{backend::errors::Error, resample::common::validate_args};

fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        let k = x * std::f32::consts::PI;
        std::primitive::f32::sin(k) / k
    }
}

pub fn resample(
    input_frame_rate_hz: u32,
    output_frame_rate_hz: u32,
    num_channels: u32,
    input_audio: &[f32],
) -> Result<Vec<f32>, Error> {
    validate_args(input_frame_rate_hz, output_frame_rate_hz, num_channels)?;

    let sample_ratio: f32 = (output_frame_rate_hz as f32) / (input_frame_rate_hz as f32);

    let ret_size: usize = ((input_audio.len() as f32) * sample_ratio).ceil() as usize;
    let mut ret: Vec<f32> = vec![0.0_f32; ret_size];

    let (mut interp_win, precision, _) = sinc_window();
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
        sample_ratio,
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
    sample_ratio: f32,
    interp_win: &[f32],
    interp_delta: &[f32],
    num_table: i32,
) {
    let scale = std::primitive::f32::min(sample_ratio, 1.0);

    // equal to (in_audio_hz / out_audio_hz)
    let time_increment = 1.0 / sample_ratio;
    let index_step = (scale * num_table as f32) as usize;

    let n_win = interp_win.len();
    let n_in_frames = in_audio.len() / (num_channels);
    let n_out_frames = out_audio.len() / (num_channels);

    for out_frame_idx in 0..n_out_frames {
        let time_register = time_increment * out_frame_idx as f32;
        let in_frame_idx = time_register as usize;

        let frac: f32 = scale * time_register.fract();
        let index_frac = frac * num_table as f32;
        let offset = index_frac as usize;
        let eta = index_frac.fract();

        let i_max = std::cmp::min(in_frame_idx + 1, (n_win - offset) / index_step);
        for i in 0..i_max {
            let weight =
                interp_win[offset + i * index_step] + eta * interp_delta[offset + i * index_step];
            for j in 0..num_channels {
                let out_idx: usize = (out_frame_idx * num_channels) + j;
                let in_idx: usize = ((in_frame_idx - i) * num_channels) + j;
                out_audio[out_idx] += weight * in_audio[in_idx];
            }
        }

        // Right wing of response
        let frac = scale - frac;
        let index_frac = frac * num_table as f32;
        let offset = index_frac as usize;
        let eta = index_frac.fract();

        let k_max = std::cmp::min(n_in_frames - in_frame_idx - 1, (n_win - offset) / index_step);
        for k in 0..k_max {
            let weight =
                interp_win[offset + k * index_step] + eta * interp_delta[offset + k * index_step];
            for j in 0..num_channels {
                let out_idx: usize = (out_frame_idx * num_channels) + j;
                let in_idx: usize = ((in_frame_idx + k + 1) * num_channels) + j;
                out_audio[out_idx] += weight * in_audio[in_idx];
            }
        }
    }
}

pub fn sinc_window() -> (Vec<f32>, i32, f32) {
    let num_zeros = 512;
    let precision = 9;
    let rolloff = 0.945;

    let num_bits = 1 << precision;
    let n = num_bits * num_zeros;

    let sinc_win = (0..=n)
        .map(|i| (num_zeros as f32) * ((i as f32) / (n as f32)) * (num_zeros as f32)) // np.linspace
        .map(|x| rolloff * sinc(x * rolloff)); // rolloff * np.sinc;

    let taper = blackman_harris(2 * n + 1).skip(n as usize);

    let interp_win = taper.zip(sinc_win).map(|(t, s)| t * s).collect::<Vec<_>>();

    return (interp_win, num_bits, rolloff);
}

pub fn blackman_harris(n: i32) -> impl Iterator<Item = f32> {
    const C1: f32 = 0.35875;
    const C2: f32 = 0.48829;
    const C3: f32 = 0.14128;
    const C4: f32 = 0.01168;

    const PI_2: f32 = std::f32::consts::TAU;
    const PI_4: f32 = std::f32::consts::TAU * 2.0;
    const PI_6: f32 = std::f32::consts::PI * 6.0;

    let aaa: f32 = n as f32 - 1.0;
    let v2 = PI_2 / aaa;
    let v3 = PI_4 / aaa;
    let v4 = PI_6 / aaa;

    return (0..n).map(move |k| {
        C1 - C2 * (v2 * k as f32).cos() + C3 * (v3 * k as f32).cos() - C4 * (v4 * k as f32).cos()
    });
}

#[cfg(test)]
mod test {
    use super::blackman_harris;

    #[test]
    fn test_blackman_harris() {
        // expected_results generated from scipy.signal.blackmanharris
        let expected_results: Vec<f32> = vec![
            6.0000000000001025e-05,
            0.015071173410218162,
            0.1470395578623815,
            0.5205749999999999,
            0.9316592687274005,
            0.9316592687274005,
            0.5205750000000002,
            0.14703955786238157,
            0.015071173410218162,
            6.0000000000001025e-05,
        ];
        let actual_results = blackman_harris(10);

        const EPSILON: f32 = 1e-6;

        for (i, (actual, expected)) in actual_results.zip(expected_results).enumerate() {
            let error = (actual - expected).abs();
            eprintln!("{} error: {}", i, error);
            assert!(error < EPSILON)
        }
    }
}
