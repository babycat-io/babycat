use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::signal::Signal;

pub struct SkipSamplesDecoderIter<D: DecoderIter> {
    iter: D,
    count: usize,
    disabled: bool,
}

impl<D: DecoderIter> SkipSamplesDecoderIter<D> {
    #[inline(always)]
    pub fn new(iter: D, count: usize) -> Self {
        let disabled: bool = count == 0;
        Self {
            iter,
            count,
            disabled,
        }
    }
}

impl<D: DecoderIter> DecoderIter for SkipSamplesDecoderIter<D> {}

impl<D: DecoderIter> Signal for SkipSamplesDecoderIter<D> {
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

impl<D: DecoderIter> Iterator for SkipSamplesDecoderIter<D> {
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
