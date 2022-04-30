mod fixtures;

mod test_append_zeros {
    use crate::fixtures::new_small_waveform;
    use babycat::Source;

    #[test]
    fn test_do_nothing() {
        let waveform = new_small_waveform();

        let ws = waveform.into_source();
        assert_eq!(ws.size_hint(), (30, Some(30)));

        let ws = ws.append_zeros(0);
        assert_eq!(ws.size_hint(), (30, Some(30)));
    }

    #[test]
    fn test_append_1() {
        let waveform = new_small_waveform();

        let ws = waveform.into_source();
        assert_eq!(ws.size_hint(), (30, Some(30)));

        let ws = ws.append_zeros(1);
        assert_eq!(ws.size_hint(), (33, Some(33)));
    }
}
