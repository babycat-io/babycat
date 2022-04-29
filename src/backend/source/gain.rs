use crate::backend::display::est_num_frames_to_str;
use crate::backend::Signal;
use crate::backend::Source;

pub struct Gain<S: Source> {
    iter: S,
    ratio: f32,
}

impl<S: Source> std::fmt::Debug for Gain<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Gain {{ {} frames,  {} channels,  {} hz,  {}; ratio: {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
            self.ratio,
        )
    }
}

impl<S: Source> Gain<S> {
    #[inline]
    pub fn new(iter: S, dbfs: f32) -> Self {
        let ratio: f32 = (10.0_f32).powf(dbfs / 20.0_f32);
        Self { iter, ratio }
    }
}

impl<S: Source> Source for Gain<S> {}

impl<S: Source> Signal for Gain<S> {
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

impl<S: Source> Iterator for Gain<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(val) => Some(val * self.ratio),
        }
    }
}
