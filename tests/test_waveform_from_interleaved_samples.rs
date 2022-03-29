mod test_waveform_from_interleaved_samples {
    use babycat::Signal;
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

    #[test]
    fn test_five_frames_three_channels_1() {
        let interleaved_samples: Vec<f32> = vec![
            -1.0, -0.9, -0.8, //
            -0.7, -0.6, -0.5, //
            -0.4, -0.3, -0.2, //
            -0.1, 0.0, 0.1, //
            0.2, 0.3, 0.4,
        ];
        let waveform = Waveform::from_interleaved_samples(44100, 3, &interleaved_samples);
        assert_eq!(waveform.num_channels(), 3);
        assert_eq!(waveform.num_frames(), 5);
        assert_eq!(waveform.frame_rate_hz(), 44100);
        //
        // Try fetching nonexistent values and receive a None response.
        assert_eq!(waveform.get_sample(0, 3), None);
        assert_eq!(waveform.get_sample(5, 0), None);
        assert_eq!(waveform.get_sample(5, 3), None);
        //
        // Fetch every single value with get_sample().
        assert_eq!(waveform.get_sample(0, 0).unwrap(), -1.0);
        assert_eq!(waveform.get_sample(0, 1).unwrap(), -0.9);
        assert_eq!(waveform.get_sample(0, 2).unwrap(), -0.8);
        //
        assert_eq!(waveform.get_sample(1, 0).unwrap(), -0.7);
        assert_eq!(waveform.get_sample(1, 1).unwrap(), -0.6);
        assert_eq!(waveform.get_sample(1, 2).unwrap(), -0.5);
        //
        assert_eq!(waveform.get_sample(2, 0).unwrap(), -0.4);
        assert_eq!(waveform.get_sample(2, 1).unwrap(), -0.3);
        assert_eq!(waveform.get_sample(2, 2).unwrap(), -0.2);
        //
        assert_eq!(waveform.get_sample(3, 0).unwrap(), -0.1);
        assert_eq!(waveform.get_sample(3, 1).unwrap(), 0.0);
        assert_eq!(waveform.get_sample(3, 2).unwrap(), 0.1);
        //
        assert_eq!(waveform.get_sample(4, 0).unwrap(), 0.2);
        assert_eq!(waveform.get_sample(4, 1).unwrap(), 0.3);
        assert_eq!(waveform.get_sample(4, 2).unwrap(), 0.4);
        //
        //
        unsafe {
            assert_eq!(waveform.get_unchecked_sample(0, 0), -1.0);
            assert_eq!(waveform.get_unchecked_sample(0, 1), -0.9);
            assert_eq!(waveform.get_unchecked_sample(0, 2), -0.8);

            assert_eq!(waveform.get_unchecked_sample(1, 0), -0.7);
            assert_eq!(waveform.get_unchecked_sample(1, 1), -0.6);
            assert_eq!(waveform.get_unchecked_sample(1, 2), -0.5);

            assert_eq!(waveform.get_unchecked_sample(2, 0), -0.4);
            assert_eq!(waveform.get_unchecked_sample(2, 1), -0.3);
            assert_eq!(waveform.get_unchecked_sample(2, 2), -0.2);

            assert_eq!(waveform.get_unchecked_sample(3, 0), -0.1);
            assert_eq!(waveform.get_unchecked_sample(3, 1), 0.0);
            assert_eq!(waveform.get_unchecked_sample(3, 2), 0.1);

            assert_eq!(waveform.get_unchecked_sample(4, 0), 0.2);
            assert_eq!(waveform.get_unchecked_sample(4, 1), 0.3);
            assert_eq!(waveform.get_unchecked_sample(4, 2), 0.4);
        }
        let interleaved_samples_2: Vec<f32> = waveform.to_interleaved_samples().to_vec();
        assert_eq!(interleaved_samples, interleaved_samples_2);
    }
}
