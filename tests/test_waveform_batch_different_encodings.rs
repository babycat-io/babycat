// This test file checks that the "same" raw audio file encoded
// to a bunch of different codecs will all decode to roughly
// the sawm raw audio.

mod test_waveform_different_encodings {
    use babycat::batch::waveforms_from_files;
    use babycat::Signal;
    // I am not certain why the same audio file encoded as MP3, WAV, or FLAC
    // ends up with significantly different numbers of samples.
    // The values for num_frames() in unit tests like these might have to change
    // if we make the different Symphonia decoders behave like each other.
    #[test]
    fn test_different_encodings_1() {
        // TODO(jamesmishra): Find out why some MP3 files still have different frame rates.
        let mp3_filenames = &[
            "./audio-for-tests/log-sweep/192kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/224kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/256kbps-cbr.mp3",
            //"./audio-for-tests/log-sweep/320kbps-cbr.mp3",
            "./audio-for-tests/log-sweep/variable-highest.mp3",
            "./audio-for-tests/log-sweep/variable-high.mp3",
            //"./audio-for-tests/log-sweep/variable-medium-high.mp3",
            "./audio-for-tests/log-sweep/variable-medium-low.mp3",
            "./audio-for-tests/log-sweep/variable-low.mp3",
        ];
        for named_result in
            waveforms_from_files(mp3_filenames, Default::default(), Default::default())
        {
            let expected_frame_rate = 44100;
            let expected_num_channels = 2;
            let expected_num_frames = 443520;

            let name = named_result.name;
            let waveform = named_result.result.unwrap();

            let actual_frame_rate = waveform.frame_rate_hz();
            assert_eq!(
                actual_frame_rate, expected_frame_rate,
                "Wrong frame rate for {}. Got {}. Expected {}.",
                name, actual_frame_rate, expected_frame_rate
            );
            assert_eq!(waveform.num_channels(), expected_num_channels);

            let actual_num_channels = waveform.num_channels();
            assert_eq!(
                actual_num_channels, expected_num_channels,
                "Wrong num channels for {}. Got {}. Expected {}.",
                name, actual_num_channels, expected_num_channels
            );
            let actual_num_frames = waveform.num_frames();
            assert_eq!(
                actual_num_frames, expected_num_frames,
                "Wrong num frames for {}. Got {}. Expected {}.",
                name, actual_num_channels, expected_num_channels
            );
        }
        let flac_filenames = &[
            "./audio-for-tests/log-sweep/pcm-16.flac",
            "./audio-for-tests/log-sweep/pcm-24.flac",
            "./audio-for-tests/log-sweep/pcm-8.flac",
        ];
        for named_result in
            waveforms_from_files(flac_filenames, Default::default(), Default::default())
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
            waveforms_from_files(wav_filenames, Default::default(), Default::default())
        {
            println!("{}", named_result.name);
            let waveform = named_result.result.unwrap();
            assert_eq!(waveform.frame_rate_hz(), 44100);
            assert_eq!(waveform.num_channels(), 2);
            assert_eq!(waveform.num_frames(), 441000);
        }
    }
}
