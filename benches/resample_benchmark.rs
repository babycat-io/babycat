use criterion::{criterion_group, criterion_main, Criterion};

fn resample_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("resample_benchmark");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(15);

    let audio = babycat::FloatWaveform::from_file(
        "./audio-for-tests/blippy-trance/track.wav",
        Default::default(),
    )
    .unwrap();

    group.bench_function("lanczos_resample", |b| {
        b.iter(|| audio.resample_by_mode(48000, babycat::RESAMPLE_MODE_LANCZOS))
    });

    group.bench_function("libsamplerate_resample", |b| {
        b.iter(|| audio.resample_by_mode(48000, babycat::RESAMPLE_MODE_LIBSAMPLERATE))
    });
}

criterion_group!(benches, resample_benchmark);
criterion_main!(benches);
