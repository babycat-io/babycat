mod fixtures;

mod test_prepend_zeros {
    use crate::fixtures::new_small_waveform;
    use babycat::Source;

    #[test]
    fn test_do_nothing() {
        let waveform = new_small_waveform();

        let ws = waveform.into_source();
        assert_eq!(ws.size_hint(), (30, Some(30)));

        let ws = ws.prepend_zeros(0);
        assert_eq!(ws.size_hint(), (30, Some(30)));
    }

    #[test]
    fn test_prepend_1() {
        let waveform = new_small_waveform();

        let ws = waveform.into_source().prepend_zeros(1);
        assert_eq!(ws.size_hint(), (33, Some(33)));
    }
}
