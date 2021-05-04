use criterion::{criterion_group, criterion_main, Criterion};

fn resample_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("resample_benchmarks");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(15);

    let wav_reader = hound::WavReader::open("../audio-for-tests/blippy-trance/track.wav").unwrap();
    let input_audio: Vec<f32> = wav_reader
        .into_samples::<f32>()
        .map(|s| s.unwrap())
        .collect();

    group.bench_function("lanczos_resample", |b| {
        b.iter(|| babycat_lib_resample::lanczos::resample(44100, 48000, 2, &input_audio))
    });

    group.bench_function("libsamplerate_resample", |b| {
        b.iter(|| babycat_lib_resample::libsamplerate::resample(44100, 48000, 2, &input_audio))
    });
}

criterion_group!(benches, resample_benchmarks);
criterion_main!(benches);
