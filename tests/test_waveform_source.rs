mod fixtures;

mod test_waveform_source {
    use crate::fixtures::*;

    use babycat::source::WaveformSource;
    use babycat::{Signal, Waveform};

    pub const COF_NUM_SAMPLES: usize = COF_NUM_FRAMES * COF_NUM_CHANNELS as usize;

    #[inline(always)]
    #[track_caller]
    fn assert_ws(ws: &WaveformSource, num_channels: u16, frame_rate_hz: u32, num_samples: usize) {
        let num_frames = num_samples / num_channels as usize;
        assert_eq!(num_channels, ws.num_channels(), "Wrong number of channels");
        assert_eq!(frame_rate_hz, ws.frame_rate_hz(), "Wrong frame rate");
        assert_eq!(
            num_frames,
            ws.num_frames_estimate().unwrap(),
            "Wrong num_frames_estimate"
        );
        assert_eq!(num_samples, ws.size_hint().0, "Wrong size hint lower bound");
        assert_eq!(
            num_samples,
            ws.size_hint().1.unwrap(),
            "Wrong size hint upper bound"
        );
    }

    #[test]
    fn test_circus_of_freaks_default_1() {
        let waveform = Waveform::from_file(COF_FILENAME, Default::default()).unwrap();
        let mut ws = waveform.to_source();
        assert_ws(&ws, COF_NUM_CHANNELS, COF_FRAME_RATE_HZ, COF_NUM_SAMPLES);
        ws.next();
        assert_ws(
            &ws,
            COF_NUM_CHANNELS,
            COF_FRAME_RATE_HZ,
            COF_NUM_SAMPLES - 1,
        );
        ws.next();
        assert_ws(
            &ws,
            COF_NUM_CHANNELS,
            COF_FRAME_RATE_HZ,
            COF_NUM_SAMPLES - 2,
        );
        ws.nth(0);
        assert_ws(
            &ws,
            COF_NUM_CHANNELS,
            COF_FRAME_RATE_HZ,
            COF_NUM_SAMPLES - 3,
        );
    }

    #[test]
    fn test_from_frames_of_silence() {
        let frame_rate_hz: u32 = 1234;
        let num_channels: u16 = 3;
        let num_frames: usize = 101;
        let num_samples: usize = num_frames * num_channels as usize;
        let waveform = Waveform::from_frames_of_silence(frame_rate_hz, num_channels, num_frames);
        let ws = waveform.to_source();
        assert_ws(&ws, num_channels, frame_rate_hz, num_samples);
    }
}
