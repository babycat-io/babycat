use crate::backend::display::est_num_frames_to_str;
use crate::backend::units::frames_to_samples;
use crate::backend::Signal;
use crate::backend::Source;

/// [`Source::sum()`]
pub struct Sum<S1: Source, S2: Source> {
    first: S1,
    second: S2,
    remaining_samples: usize,
}

impl<S1: Source, S2: Source> std::fmt::Debug for Sum<S1, S2> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sum {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S1: Source, S2: Source> Sum<S1, S2> {
    #[inline]
    pub fn new(first: S1, second: S2, offset_frames: usize) -> Self {
        let remaining_samples = frames_to_samples(offset_frames, first.num_channels());
        Self {
            first,
            second,
            remaining_samples,
        }
    }
}

impl<S1: Source, S2: Source> Source for Sum<S1, S2> {}

impl<S1: Source, S2: Source> Signal for Sum<S1, S2> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.first.frame_rate_hz()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.first.num_channels()
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        match (
            self.first.num_frames_estimate(),
            self.second.num_frames_estimate(),
        ) {
            (Some(first), Some(second)) => Some(first + second),
            _ => None,
        }
    }
}

impl<S1: Source, S2: Source> Iterator for Sum<S1, S2> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (first_lower, first_upper) = self.first.size_hint();
        let (second_lower, second_upper) = self.second.size_hint();
        let lower = std::cmp::min(first_lower, second_lower);
        let upper = match (first_upper, second_upper) {
            (Some(first), Some(second)) => Some(std::cmp::max(first, second)),
            _ => None,
        };
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_samples > 0 {
            self.remaining_samples -= 1;
            return self.first.next();
        }
        match (self.first.next(), self.second.next()) {
            (Some(first), Some(second)) => Some(first + second),
            (Some(first), None) => Some(first),
            (None, Some(second)) => Some(second),
            _ => None,
        }
    }
}
