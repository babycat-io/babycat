mod test_int_waveform {
    use babycat::{FloatWaveform, IntWaveform, Waveform};

    fn generate_int_samples() -> Vec<i16> {
        (-0x8000..0x8000).map(|v| v as i16).collect()
    }

    #[test]
    fn test_bit_transparency_1() {
        // Create an IntWaveform.
        let int_samples: Vec<i16> = generate_int_samples();
        let int_waveform_1 = IntWaveform::new(44100, 2, int_samples.clone());
        assert_eq!(int_waveform_1.num_frames(), 0x8000);
        assert_eq!(int_waveform_1.num_channels(), 2);
        assert_eq!(int_waveform_1.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_1.interleaved_samples(), int_samples);
        // Convert it to a FloatWaveform.
        let float_waveform = FloatWaveform::from(int_waveform_1.clone());
        assert_eq!(float_waveform.num_frames(), 0x8000);
        assert_eq!(float_waveform.num_channels(), 2);
        assert_eq!(float_waveform.frame_rate_hz(), 44100);
        // And convert it back to an IntWaveForm and make sure that nothing has changed.
        let int_waveform_2 = IntWaveform::from(float_waveform);
        assert_eq!(int_waveform_1, int_waveform_2);
        assert_eq!(int_waveform_2.num_frames(), 0x8000);
        assert_eq!(int_waveform_2.num_channels(), 2);
        assert_eq!(int_waveform_2.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_2.interleaved_samples(), int_samples);
    }

    #[test]
    fn test_bit_transparency_silence_1() {
        let int_waveform_1 =
            IntWaveform::from(FloatWaveform::from_milliseconds_of_silence(44100, 2, 30000));
        let int_waveform_2 = IntWaveform::from(FloatWaveform::from(int_waveform_1.clone()));
        assert_eq!(int_waveform_1.num_frames(), 1323000);
        assert_eq!(int_waveform_1.num_channels(), 2);
        assert_eq!(int_waveform_1.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_2.num_frames(), 1323000);
        assert_eq!(int_waveform_2.num_channels(), 2);
        assert_eq!(int_waveform_2.frame_rate_hz(), 44100);
    }

    #[test]
    fn test_serialize_1() {
        // Create an IntWaveform.
        let int_samples: Vec<i16> = generate_int_samples();
        let int_waveform_1 = IntWaveform::new(44100, 2, int_samples.clone());
        // Serialize and deserialize the IntWaveform.
        let int_waveform_json: String = serde_json::to_string(&int_waveform_1).unwrap();
        let int_waveform_deserialized: IntWaveform =
            serde_json::from_str(&int_waveform_json).unwrap();
        // Convert it to a FloatWaveform. Serialize and deserialize it.
        let float_waveform = FloatWaveform::from(int_waveform_1.clone());
        let float_waveform_json: String = serde_json::to_string(&float_waveform).unwrap();
        let float_waveform_deserialized: FloatWaveform =
            serde_json::from_str(&float_waveform_json).unwrap();
        // And convert it back to an IntWaveForm and make sure that nothing has changed.
        // Check that all of our IntWaveforms are the same.
        let int_waveform_2 = IntWaveform::from(float_waveform.clone());
        let int_waveform_2_from_float_waveform_deserialized =
            IntWaveform::from(float_waveform_deserialized);
        assert_eq!(int_waveform_1, int_waveform_2);
        assert_eq!(int_waveform_1, int_waveform_deserialized);
        assert_eq!(
            int_waveform_2,
            int_waveform_2_from_float_waveform_deserialized
        );
    }
}
