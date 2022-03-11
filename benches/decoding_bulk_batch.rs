use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_bulk_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_bulk_batch");

    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(180));

    let filenames = &[
        "./audio-for-tests/andreas-theme/track.flac",
        "./audio-for-tests/blippy-trance/track.wav",
        "./audio-for-tests/circus-of-freaks/track.flac",
        "./audio-for-tests/left-channel-tone/track.flac",
        "./audio-for-tests/mono-dtmf-tones/track.flac",
        "./audio-for-tests/on-hold-for-you/track.flac",
        "./audio-for-tests/tone-missing-sounds/track.flac",
        "./audio-for-tests/voxel-revolution/track.flac",
    ];

    group.bench_function("old", |b| {
        b.iter(|| {
            for _i in 0..20 {
                let batch = babycat::batch::waveforms_from_files(
                    filenames,
                    Default::default(),
                    Default::default(),
                );
                for named_result in batch {
                    let _waveform = named_result.result.unwrap();
                }
            }
        })
    });

    group.bench_function("new", |b| {
        b.iter(|| {
            let batch = babycat::batch::Batch::new(Default::default());
            for _i in 0..20 {
                let batch = batch.waveforms_from_files(
                    filenames,
                    Default::default(),
                );
                for named_result in batch {
                    let _waveform = named_result.result.unwrap();
                }
            }
        })
    });
}

criterion_group!(benches, decoding_bulk_batch);
criterion_main!(benches);
