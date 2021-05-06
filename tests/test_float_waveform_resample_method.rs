mod fixtures;

mod test_float_waveform_resample_method {
    use crate::fixtures::*;

    use babycat::DecodeArgs;
    use babycat::Error;
    use babycat::FloatWaveform;
    use babycat::Waveform;

    fn decode(
        filename: &str,
        decode_args: DecodeArgs,
        frame_rate_hz: u32,
        resample_mode: u32,
    ) -> Result<FloatWaveform, Error> {
        let waveform = FloatWaveform::from_file(filename, decode_args).unwrap();
        waveform.resample_by_mode(frame_rate_hz, resample_mode)
    }

    #[test]
    fn test_circus_of_freaks_44099() {
        let resampled = decode(
            COF_FILENAME,
            Default::default(),
            44099,
            babycat::DEFAULT_RESAMPLE_MODE,
        )
        .unwrap();
        assert_eq!(resampled.num_channels(), 2);
        assert_eq!(resampled.num_frames(), 2492872);
        assert_eq!(resampled.frame_rate_hz(), 44099);
    }

    #[test]
    fn test_circus_of_freaks_22050() {
        let resampled = decode(
            COF_FILENAME,
            Default::default(),
            22050,
            babycat::DEFAULT_RESAMPLE_MODE,
        )
        .unwrap();
        assert_eq!(resampled.num_channels(), 2);
        assert_eq!(resampled.num_frames(), COF_NUM_FRAMES / 2);
        assert_eq!(resampled.frame_rate_hz(), 22050);
    }

    #[test]
    fn test_circus_of_freaks_11025() {
        let resampled = decode(
            COF_FILENAME,
            Default::default(),
            11025,
            babycat::DEFAULT_RESAMPLE_MODE,
        )
        .unwrap();
        assert_eq!(resampled.num_channels(), 2);
        assert_eq!(resampled.num_frames(), COF_NUM_FRAMES / 4);
        assert_eq!(resampled.frame_rate_hz(), 11025);
    }
}
