use crate::backend::display::est_num_frames_to_str;
use crate::backend::{Signal, Source, Waveform};

#[derive(Clone, PartialEq)]
pub struct WaveformSource {
    waveform: Waveform,
    current_sample: usize,
}

impl std::fmt::Debug for WaveformSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "WaveformSource {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl WaveformSource {
    #[inline]
    pub fn from_interleaved_samples(
        frame_rate_hz: u32,
        num_channels: u16,
        interleaved_samples: &[f32],
    ) -> Self {
        Self::new(Waveform::from_interleaved_samples(
            frame_rate_hz,
            num_channels,
            interleaved_samples,
        ))
    }

    #[inline]
    pub fn new(waveform: Waveform) -> Self {
        Self {
            waveform,
            current_sample: 0,
        }
    }

    #[inline]
    pub fn remaining_samples(&self) -> usize {
        self.waveform
            .num_samples()
            .saturating_sub(self.current_sample)
    }

    #[inline]
    pub fn remaining_frames(&self) -> usize {
        self.remaining_samples() / self.waveform.num_channels() as usize
    }
}

impl From<Waveform> for WaveformSource {
    fn from(waveform: Waveform) -> WaveformSource {
        WaveformSource::new(waveform)
    }
}

impl Source for WaveformSource {}

impl Signal for WaveformSource {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.waveform.frame_rate_hz()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.waveform.num_channels()
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        Some(self.remaining_frames())
    }
}

impl Iterator for WaveformSource {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining_samples();
        (remaining, Some(remaining))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.waveform.num_samples() {
            return None;
        }
        let retval = self.waveform.get_interleaved_sample(self.current_sample);
        self.current_sample += 1;
        retval
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current_sample += n;
        self.next()
    }
}
