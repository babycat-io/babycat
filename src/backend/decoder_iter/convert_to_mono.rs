use crate::backend::signal::Signal;
use crate::backend::DecoderIter;

pub struct ConvertToMono<D: DecoderIter> {
    iter: D,
    disabled: bool,
    iter_num_channels_usize: usize,
    iter_num_channels_f32: f32,
}

impl<D: DecoderIter> ConvertToMono<D> {
    #[inline(always)]
    pub fn new(iter: D, enabled: bool) -> Self {
        let iter_num_channels_usize: usize = iter.num_channels() as usize;
        let iter_num_channels_f32: f32 = iter_num_channels_usize as f32;
        Self {
            iter,
            disabled: !enabled,
            iter_num_channels_usize,
            iter_num_channels_f32,
        }
    }
}

impl<D: DecoderIter> DecoderIter for ConvertToMono<D> {}

impl<D: DecoderIter> Signal for ConvertToMono<D> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.iter.frame_rate_hz()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        1
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.iter.num_frames_estimate()
    }
}

impl<D: DecoderIter> Iterator for ConvertToMono<D> {
    type Item = f32;

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter.size_hint() {
            (min, None) => (min / self.iter_num_channels_usize, None),
            (min, Some(max)) => (
                min / self.iter_num_channels_usize,
                Some(max / self.iter_num_channels_usize),
            ),
        }
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.disabled {
            return self.iter.next();
        }
        let mut psum: f32 = 0.0_f32;
        for _ in 0..self.iter_num_channels_usize {
            match self.iter.next() {
                None => return None,
                Some(val) => psum += val,
            }
        }
        Some(psum / self.iter_num_channels_f32)
    }
}
