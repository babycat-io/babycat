fn main() {
    let filenames = &[
        "./audio-for-tests/andreas-theme/track.mp3",
        "./audio-for-tests/blippy-trance/track.mp3",
        "./audio-for-tests/circus-of-freaks/track.mp3",
        "./audio-for-tests/left-channel-tone/track.mp3",
        "./audio-for-tests/mono-dtmf-tones/track.mp3",
        "./audio-for-tests/on-hold-for-you/track.mp3",
        "./audio-for-tests/tone-missing-sounds/track.mp3",
        "./audio-for-tests/voxel-revolution/track.mp3",
    ];
    let batch =
        babycat::batch::waveforms_from_files(filenames, Default::default(), Default::default());
    for named_result in batch {
        let _waveform = named_result.result.unwrap();
    }
}
