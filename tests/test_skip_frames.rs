mod fixtures;

mod test_skip_frames {
    use crate::fixtures::new_small_waveform;
    use babycat::Source;

    #[test]
    fn test_size_hint_1() {
        let waveform = new_small_waveform();

        let ws = waveform.into_source();
        assert_eq!(ws.size_hint(), (30, Some(30)));

        let mut ws = ws.skip_frames(1);
        assert_eq!(ws.size_hint(), (27, Some(27)));

        ws.next();
        assert_eq!(ws.size_hint(), (26, Some(26)));

        let ws = ws.skip_frames(10);
        assert_eq!(ws.size_hint(), (0, Some(0)));
    }
}
