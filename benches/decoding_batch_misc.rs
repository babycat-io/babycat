use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_batch_misc(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_batch_misc");

    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(90));

    let batch_1_filenames = &[
        "./audio-for-tests/andreas-theme/track.mp3",
        "./audio-for-tests/blippy-trance/track.mp3",
        "./audio-for-tests/circus-of-freaks/track.mp3",
        "./audio-for-tests/left-channel-tone/track.mp3",
        "./audio-for-tests/mono-dtmf-tones/track.mp3",
        "./audio-for-tests/on-hold-for-you/track.mp3",
        "./audio-for-tests/tone-missing-sounds/track.mp3",
        "./audio-for-tests/voxel-revolution/track.mp3",
    ];

    group.bench_function("batch_1", |b| {
        b.iter(|| {
            let batch = babycat::batch::waveforms_from_files(
                batch_1_filenames,
                Default::default(),
                Default::default(),
            );
            for named_result in batch {
                let _waveform = named_result.result.unwrap();
            }
        })
    });
}

criterion_group!(benches, decoding_batch_misc);
criterion_main!(benches);
