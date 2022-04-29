use crate::backend::display::est_num_frames_to_str;
use crate::backend::Signal;
use crate::backend::Source;

pub struct Scale<S: Source> {
    iter: S,
    constant: f32,
}

impl<S: Source> std::fmt::Debug for Scale<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Scale {{ {} frames,  {} channels,  {} hz,  {}; constant: {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
            self.constant,
        )
    }
}

impl<S: Source> Scale<S> {
    #[inline]
    pub fn new(iter: S, constant: f32) -> Self {
        Self { iter, constant }
    }
}

impl<S: Source> Source for Scale<S> {}

impl<S: Source> Signal for Scale<S> {
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

impl<S: Source> Iterator for Scale<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| v * self.constant)
    }
}
