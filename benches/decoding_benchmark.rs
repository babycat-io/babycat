use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_benchmark");

    group.significance_level(0.01);

    let symphonia_args =
        babycat::WaveformArgs::default().set_decoding_backend(babycat::DECODING_BACKEND_SYMPHONIA);

    group.bench_function("andreas-theme", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/andreas-theme/track.mp3",
                symphonia_args,
            )
            .unwrap()
        })
    });

    group.bench_function("blippy-trance", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/blippy-trance/track.mp3",
                symphonia_args,
            )
            .unwrap()
        })
    });

    group.bench_function("circus-of-freaks", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/circus-of-freaks/track.mp3",
                symphonia_args,
            )
            .unwrap()
        })
    });

    group.bench_function("mono-dtmf-tones", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/mono-dtmf-tones/track.mp3",
                symphonia_args,
            )
            .unwrap()
        })
    });

    group.bench_function("voxel-revolution", |b| {
        b.iter(|| {
            babycat::Waveform::from_file(
                "./audio-for-tests/voxel-revolution/track.mp3",
                symphonia_args,
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, decoding_benchmark);
criterion_main!(benches);
