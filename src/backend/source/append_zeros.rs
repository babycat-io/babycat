use crate::backend::display::est_num_frames_to_str;
use crate::backend::units::frames_to_samples;
use crate::backend::Signal;
use crate::backend::Source;

/// [`Source::append_zeros()`]
pub struct AppendZeros<S: Source> {
    iter: S,
    drained: bool,
    num_samples_remaining: usize,
}

impl<S: Source> std::fmt::Debug for AppendZeros<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AppendZeros {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S: Source> AppendZeros<S> {
    #[inline]
    pub fn new(iter: S, num_frames: usize) -> Self {
        let num_samples_remaining = frames_to_samples(num_frames, iter.num_channels());
        Self {
            iter,
            drained: false,
            num_samples_remaining,
        }
    }
}

impl<S: Source> Source for AppendZeros<S> {}

impl<S: Source> Signal for AppendZeros<S> {
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

impl<S: Source> Iterator for AppendZeros<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.drained {
            return (self.num_samples_remaining, Some(self.num_samples_remaining));
        }
        let (lower, upper) = self.iter.size_hint();
        let new_lower = lower + self.num_samples_remaining;
        let new_upper = upper.map(|u| u + self.num_samples_remaining);
        (new_lower, new_upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.num_samples_remaining == 0 {
            return None;
        }
        if self.drained {
            self.num_samples_remaining -= 1;
            return Some(0.0_f32);
        }
        let some_val = self.iter.next();
        if some_val.is_none() {
            self.drained = true;
            self.num_samples_remaining -= 1;
            return Some(0.0_f32);
        }
        some_val
    }
}
