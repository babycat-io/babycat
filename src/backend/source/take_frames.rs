use crate::backend::display::est_num_frames_to_str;
use crate::backend::units::frames_to_samples;
use crate::backend::Signal;
use crate::backend::Source;

/// [`Source::take_frames()`]
pub struct TakeFrames<S: Source> {
    iter: S,
    remaining_samples: usize,
}

impl<S: Source> std::fmt::Debug for TakeFrames<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TakeFrames {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S: Source> TakeFrames<S> {
    #[inline]
    pub fn new(iter: S, offset_frames: usize) -> Self {
        let remaining_samples = frames_to_samples(offset_frames, iter.num_channels());
        Self {
            iter,
            remaining_samples,
        }
    }
}

impl<S: Source> Source for TakeFrames<S> {}

impl<S: Source> Signal for TakeFrames<S> {
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

impl<S: Source> Iterator for TakeFrames<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = std::cmp::min(lower, self.remaining_samples);
        let upper = upper.map(|u| std::cmp::min(u, self.remaining_samples));
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_samples == 0 {
            return None;
        }
        self.remaining_samples -= 1;
        self.iter.next()
    }
}
