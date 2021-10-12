// This test file checks that the "same" raw audio file encoded
// to a bunch of different codecs will all decode to roughly
// the sawm raw audio.

mod test_float_waveform_different_encodings {
    use babycat::FloatWaveform;
    // I am not certain why the same audio file encoded as MP3, WAV, or FLAC
    // ends up with significantly different numbers of samples.
    // The values for num_frames() in unit tests like these might have to change
    // if we make the different Symphonia decoders behave like each other.
    #[test]
    fn test_different_encodings_1() {
        let mp3_filenames = &[
            "./audio-for-tests/log-sweep/192kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/224kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/256kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/320kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/variable-highest.mp3",
            "./audio-for-tests/log-sweep/variable-high.mp3",
            "./audio-for-tests/log-sweep/variable-medium-high.mp3",
            "./audio-for-tests/log-sweep/variable-medium-low.mp3",
            "./audio-for-tests/log-sweep/variable-low.mp3",
        ];
        for named_result in
            FloatWaveform::from_many_files(mp3_filenames, Default::default(), Default::default())
        {
            println!("{}", named_result.name);
            let waveform = named_result.result.unwrap();
            assert_eq!(waveform.frame_rate_hz(), 44100);
            assert_eq!(waveform.num_channels(), 2);
            assert_eq!(waveform.num_frames(), 443520);
        }
        let flac_filenames = &[
            "./audio-for-tests/log-sweep/pcm-16.flac",
            "./audio-for-tests/log-sweep/pcm-24.flac",
            "./audio-for-tests/log-sweep/pcm-8.flac",
        ];
        for named_result in
            FloatWaveform::from_many_files(flac_filenames, Default::default(), Default::default())
        {
            println!("{}", named_result.name);
            let waveform = named_result.result.unwrap();
            assert_eq!(waveform.frame_rate_hz(), 44100);
            assert_eq!(waveform.num_channels(), 2);
            assert_eq!(waveform.num_frames(), 441000);
        }
        let wav_filenames = &[
            "./audio-for-tests/log-sweep/f32.wav",
            "./audio-for-tests/log-sweep/f64.wav",
            "./audio-for-tests/log-sweep/i32.wav",
        ];
        for named_result in
            FloatWaveform::from_many_files(wav_filenames, Default::default(), Default::default())
        {
            println!("{}", named_result.name);
            let waveform = named_result.result.unwrap();
            assert_eq!(waveform.frame_rate_hz(), 44100);
            assert_eq!(waveform.num_channels(), 2);
            //assert_eq!(waveform.num_frames(), 442049);
        }
    }
}
