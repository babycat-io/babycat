mod test_waveform_args {
    use babycat::WaveformArgs;

    /// Test the default values for WaveformArgs.
    #[test]
    fn test_default() {
        let args: WaveformArgs = Default::default();
        assert_eq!(args.start_time_milliseconds, 0);
        assert_eq!(args.end_time_milliseconds, 0);
        assert_eq!(args.frame_rate_hz, 0);
        assert_eq!(args.num_channels, 0);
        assert_eq!(args.convert_to_mono, false);
        assert_eq!(args.zero_pad_ending, false);
        assert_eq!(args.resample_mode, 0);
        assert_eq!(args.decoding_backend, 0);
    }

    /// Test the getters and setters for WaveformArgs`.
    #[test]
    fn test_setters() {
        let args = WaveformArgs::default()
            .set_start_time_milliseconds(1000)
            .set_end_time_milliseconds(2000)
            .set_frame_rate_hz(44100)
            .set_num_channels(3)
            .set_convert_to_mono(true)
            .set_zero_pad_ending(true)
            .set_resample_mode(1)
            .set_decoding_backend(1);
        assert_eq!(args.start_time_milliseconds, 1000);
        assert_eq!(args.end_time_milliseconds, 2000);
        assert_eq!(args.frame_rate_hz, 44100);
        assert_eq!(args.num_channels, 3);
        assert_eq!(args.convert_to_mono, true);
        assert_eq!(args.zero_pad_ending, true);
        assert_eq!(args.resample_mode, 1);
        assert_eq!(args.decoding_backend, 1);
    }
}
