use crate::backend::Signal;
use crate::backend::Source;
use crate::backend::display::est_num_frames_to_str;

pub struct StackChannels<S1: Source, S2: Source> {
    first: S1,
    second: S2,
    first_exhausted: bool,
    second_exhausted: bool,
    current_source_is_first: bool,
    total_num_channels: usize,
    current_channel: usize,
}

impl<S: Source> std::fmt::Debug for StackChannels<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "StackChannels {{ {} frames,  {} + {} = {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.first.num_channels(), self.second.num_channels(), self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),

        )
    }
}

impl<S1: Source, S2: Source> StackChannels<S1, S2> {
    #[inline]
    pub fn new(first: S1, second: S2) -> Self {
        let total_num_channels = first.num_channels() + second.num_channels();
        Self {
            first,
            second,
            first_exhausted: false,
            second_exhausted: false,
            current_source_is_first: true,
            total_num_channels,
            current_channel: 0,
        }
    }
}

impl<S1: Source, S2: Source> Source for StackChannels<S1, S2> {

}


impl<S1: Source, S2: Source> Signal for StackChannels<S1, S2> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.first.frame_rate_hz()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.total_num_channels
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


impl<S1: Source, S2: Source> Iterator for StackChannels<S1, S2> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (first_lower, first_upper) = self.first.size_hint();
        let (second_lower, second_upper) = self.second.size_hint();
        let lower = first_lower + second_lower;
        let upper = match (first_upper, second_upper) {
            (Some(first), Some(second)) => Some(first + second),
            _ => None,
        };
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first_exhausted && self.second_exhausted {
            return None;
        }
        let current_channel = self.current_channel;
        self.current_channel = (self.current_channel + 1) % (self.total_num_channels);
        if current_channel < self.first.num_channels() {
            let val = self.first.next();
            if val.is_none() {
                self.first_exhausted = true;
            }
            return val;
        }
        let val = self.second.next();
        if val.is_none() {
            self.second_exhausted = true;
        }
        return val;
    }
}
