use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn decoding_log_sweep(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_log_sweep");

    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(30));

    let filenames = &[
        "192kbps-cbr.mp3",
        "224kbps-cbr.mp3",
        "256kbps-cbr.mp3",
        "320kbps-cbr.mp3",
        "variable-highest.mp3",
        "variable-high.mp3",
        "variable-medium-high.mp3",
        "variable-medium-low.mp3",
        "variable-low.mp3",
        "pcm-16.flac",
        "pcm-24.flac",
        "pcm-8.flac",
        "f32.wav",
        "f64.wav",
        "i32.wav",
    ];
    for filename in filenames {
        let full_path = format!("./audio-for-tests/log-sweep/{}", filename);
        group.bench_function(*filename, |b| {
            b.iter(|| babycat::Waveform::from_file(&full_path, Default::default()).unwrap())
        });
    }
}

criterion_group!(benches, decoding_log_sweep);
criterion_main!(benches);
