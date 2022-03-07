use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_benchmark");

    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(30));

    group.bench_function("andreas-theme", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/andreas-theme/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("blippy-trance", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/blippy-trance/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("circus-of-freaks", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/circus-of-freaks/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("left-channel-tone", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/left-channel-tone/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("mono-dtmf-tones", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/mono-dtmf-tones/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("on-hold-for-you", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/on-hold-for-you/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("tone-missing-sounds", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/tone-missing-sounds/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });

    group.bench_function("voxel-revolution", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/voxel-revolution/track.mp3",
                Default::default(),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
