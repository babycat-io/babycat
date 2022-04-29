use crate::backend::display::est_num_frames_to_str;
use crate::backend::Error;
use crate::backend::Signal;
use crate::backend::Source;

pub struct Append<S1: Source, S2: Source> {
    first: S1,
    second: S2,
}

impl<S1: Source, S2: Source> std::fmt::Debug for Append<S1, S2> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Append {{ {} + {} = {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.first.num_frames_estimate()),
            est_num_frames_to_str(self.second.num_frames_estimate()),
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S1: Source, S2: Source> Append<S1, S2> {
    #[inline]
    pub fn new(first: S1, second: S2) -> Result<Self, Error> {
        let f_nc = first.num_channels();
        let s_nc = second.num_channels();
        if f_nc != s_nc {
            return Err(Error::CannotAppendSourcesWithDifferentNumChannels(
                f_nc, s_nc,
            ));
        }
        let f_fr = first.frame_rate_hz();
        let s_fr = second.frame_rate_hz();
        if f_fr != s_fr {
            return Err(Error::CannotAppendSourcesWithDifferentFrameRates(
                f_fr, s_fr,
            ));
        }
        Ok(Self { first, second })
    }
}

impl<S1: Source, S2: Source> Source for Append<S1, S2> {}

impl<S1: Source, S2: Source> Signal for Append<S1, S2> {
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

impl<S1: Source, S2: Source> Iterator for Append<S1, S2> {
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
        match self.first.next() {
            Some(first) => Some(first),
            None => self.second.next(),
        }
    }
}
