mod fixtures;

mod test_waveform_from_encoded_stream {
    use crate::fixtures::*;
    use babycat::Waveform;
    use std::io::Cursor;

    #[test]
    fn test_circus_of_freaks_default_1() {
        let bytes = std::fs::read(COF_FILENAME).unwrap();
        let cursor = Cursor::new(bytes);
        let waveform = Waveform::from_encoded_stream(cursor, Default::default()).unwrap();
        assert_eq!(waveform.num_channels(), COF_NUM_CHANNELS);
        assert_eq!(waveform.num_frames(), COF_NUM_FRAMES);
        assert_eq!(waveform.frame_rate_hz(), COF_FRAME_RATE_HZ)
    }
}
