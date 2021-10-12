mod test_waveform_from_interleaved_samples {
    use babycat::Waveform;

    #[test]
    fn test_four_frames_three_channels() {
        let interleaved_samples: Vec<f32> = vec![
            -1.0, 0.0, 1.0, //
            -1.0, 0.0, 1.0, //
            -1.0, 0.0, 1.0, //
            -1.0, 0.0, 1.0,
        ];
        let waveform = Waveform::from_interleaved_samples(44100, 3, &interleaved_samples);
        assert_eq!(waveform.num_channels(), 3);
        assert_eq!(waveform.num_frames(), 4);
        assert_eq!(waveform.frame_rate_hz(), 44100);
    }
}
