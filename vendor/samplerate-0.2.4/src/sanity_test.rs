use std::cmp::PartialOrd;
use rstest::rstest;

#[rstest(n, from_rate, to_rate, n_ch, bleed_size, in_bleed_eps, out_bleed_eps,
    case(16384, 1, 2, 1, 512, 1e-6, 0.02),
    case(16384, 1, 2, 2, 512, 1e-6, 0.02),
    case(16384, 1, 2, 7, 512, 1e-6, 0.02),
    case(44100, 44100, 48000, 1, 512, 1e-6, 0.001),
    case(44100, 44100, 48000, 2, 512, 1e-6, 0.001),
    case(44100, 44100, 48000, 7, 512, 1e-6, 0.001),
)]
fn simple_resample(n: usize, from_rate: usize, to_rate: usize, n_ch: usize, bleed_size: usize, in_bleed_eps: f32, out_bleed_eps: f32) {
    let sig_freq = 128.0f64;
    let data = (0..n * n_ch).map(|i| (2.0 * std::f64::consts::PI * sig_freq * ((i / n_ch) as f64 / n as f64)).sin()).map(|x| x as f32).collect::<Vec<f32>>();
    let down_data = crate::convert(from_rate as u32, to_rate as u32, n_ch, crate::ConverterType::SincBestQuality, &data).unwrap();
    let up_data = crate::convert(to_rate as u32, from_rate as u32, n_ch, crate::ConverterType::SincBestQuality, &down_data).unwrap();
    assert_eq!(up_data.len(), ((n * to_rate + (from_rate - 1)) / from_rate * from_rate + (to_rate - 1)) / to_rate * n_ch);
    let max_diff = data.iter().enumerate().zip(up_data.iter()).map(|((i, a), b)| (i, (a - b).abs(),)).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    let max_diff_bleed = data.iter().take((n - bleed_size) * n_ch).enumerate().skip(bleed_size * n_ch).zip(up_data.iter().skip(bleed_size * n_ch)).map(|((i, a), b)| (i, (a - b).abs(),)).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    dbg!(max_diff, max_diff_bleed);
    assert!(max_diff_bleed.1 < in_bleed_eps);
    assert!(max_diff.1 < out_bleed_eps);
}
