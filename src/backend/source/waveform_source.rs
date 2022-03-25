use crate::backend::{Signal, Source};

pub struct WaveformSource<'a> {
    interleaved_samples: &'a [f32],
    frame_rate_hz: u32,
    num_channels: u16,
    current_sample: usize,
}

impl<'a> WaveformSource<'a> {
    pub fn new(interleaved_samples: &'a [f32], frame_rate_hz: u32, num_channels: u16) -> Self {
        Self {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            current_sample: 0,
        }
    }
}

impl<'a> Source for WaveformSource<'a> {}

impl<'a> Signal for WaveformSource<'a> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.num_channels
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        let remaining_samples: usize = self.interleaved_samples.len() - self.current_sample;
        Some(remaining_samples / self.num_channels as usize)
    }
}

impl<'a> Iterator for WaveformSource<'a> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let diff = self.interleaved_samples.len() - self.current_sample;
        (diff, Some(diff))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.interleaved_samples.len() {
            return None;
        }
        let retval: Option<Self::Item> = Some(self.interleaved_samples[self.current_sample]);
        self.current_sample += 1;
        retval
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current_sample += n;
        self.next()
    }
}
