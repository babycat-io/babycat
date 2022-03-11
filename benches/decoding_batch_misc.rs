use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_batch_misc(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_batch_misc");

    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(90));

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

    group.bench_function("all_defaults", |b| {
        b.iter(|| {
            let batch = babycat::batch::waveforms_from_files(
                filenames,
                Default::default(),
                Default::default(),
            );
            for named_result in batch {
                let _waveform = named_result.result.unwrap();
            }
        })
    });

    group.bench_function("single_worker", |b| {
        b.iter(|| {
            let batch_args = babycat::batch::BatchArgs {
                num_workers: 1,
                ..Default::default()
            };
            let batch =
                babycat::batch::waveforms_from_files(filenames, Default::default(), batch_args);
            for named_result in batch {
                let _waveform = named_result.result.unwrap();
            }
        })
    });

    group.bench_function("convert_to_mono", |b| {
        b.iter(|| {
            let waveform_args = babycat::WaveformArgs {
                convert_to_mono: true,
                ..Default::default()
            };
            let batch =
                babycat::batch::waveforms_from_files(filenames, waveform_args, Default::default());
            for named_result in batch {
                let _waveform = named_result.result.unwrap();
            }
        })
    });
}

criterion_group!(benches, decoding_batch_misc);
criterion_main!(benches);
