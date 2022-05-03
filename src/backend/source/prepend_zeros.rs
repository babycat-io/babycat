use crate::backend::display::est_num_frames_to_str;
use crate::backend::units::frames_to_samples;
use crate::backend::Signal;
use crate::backend::Source;

/// [`Source::prepend_zeros()`]
pub struct PrependZeros<S: Source> {
    iter: S,
    num_samples_remaining: usize,
}

impl<S: Source> std::fmt::Debug for PrependZeros<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PrependZeros {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S: Source> PrependZeros<S> {
    #[inline]
    pub fn new(iter: S, num_frames: usize) -> Self {
        let num_samples_remaining = frames_to_samples(num_frames, iter.num_channels());
        Self {
            iter,
            num_samples_remaining,
        }
    }
}

impl<S: Source> Source for PrependZeros<S> {}

impl<S: Source> Signal for PrependZeros<S> {
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

impl<S: Source> Iterator for PrependZeros<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = lower + self.num_samples_remaining;
        let upper = upper.map(|u| u + self.num_samples_remaining);
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.num_samples_remaining > 0 {
            self.num_samples_remaining -= 1;
            return Some(0.0_f32);
        }
        self.iter.next()
    }
}
