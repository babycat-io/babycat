mod fixtures;

mod test_take_frames {
    use crate::fixtures::new_small_waveform;
    use babycat::Source;

    #[test]
    fn test_size_hint_1() {
        let waveform = new_small_waveform();

        let ws = waveform.to_source().take_frames(1);

        assert_eq!(ws.size_hint(), (3, Some(3)));
    }
}
