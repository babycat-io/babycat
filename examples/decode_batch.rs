fn main() {
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
    for _i in 0..10 {
        let batch =
        babycat::batch::waveforms_from_files(filenames, Default::default(), Default::default());
        for named_result in batch {
            let _waveform = named_result.result.unwrap();
        }
    }
}
