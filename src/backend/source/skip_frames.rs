use crate::backend::display::est_num_frames_to_str;
use crate::backend::units::frames_to_samples;
use crate::backend::Signal;
use crate::backend::Source;

pub struct SkipFrames<S: Source> {
    iter: S,
    num_samples: usize,
}

impl<S: Source> std::fmt::Debug for SkipFrames<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SkipFrames {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S: Source> SkipFrames<S> {
    #[inline]
    pub fn new(iter: S, offset_frames: usize) -> Self {
        let num_samples = frames_to_samples(offset_frames, iter.num_channels());
        Self { iter, num_samples }
    }
}

impl<S: Source> Source for SkipFrames<S> {}

impl<S: Source> Signal for SkipFrames<S> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.iter.frame_rate_hz()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.iter.num_channels()
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.iter.num_frames_estimate()
    }
}

impl<S: Source> Iterator for SkipFrames<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = lower.saturating_sub(self.num_samples);
        let upper = upper.map(|u| u.saturating_sub(self.num_samples));
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.num_samples > 0 {
            self.iter.next();
            self.num_samples -= 1;
        }
        self.iter.next()
    }
}
