use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct SkipSamples<S: Source> {
    iter: S,
    count: usize,
    disabled: bool,
}

impl<S: Source> SkipSamples<S> {
    #[inline(always)]
    pub fn new(iter: S, count: usize) -> Self {
        let disabled: bool = count == 0;
        Self {
            iter,
            count,
            disabled,
        }
    }
}

impl<S: Source> Source for SkipSamples<S> {}

impl<S: Source> Signal for SkipSamples<S> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.iter.frame_rate_hz()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.iter.num_channels()
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.iter.num_frames_estimate()
    }
}

impl<S: Source> Iterator for SkipSamples<S> {
    type Item = f32;

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.disabled {
            return self.iter.next();
        }
        while self.count > 0 {
            self.iter.next();
            self.count -= 1;
        }
        self.iter.next()
    }
}
