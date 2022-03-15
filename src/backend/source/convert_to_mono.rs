use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct ConvertToMono<S: Source> {
    iter: S,
    iter_num_channels_usize: usize,
    iter_num_channels_f32: f32,
}

impl<S: Source> ConvertToMono<S> {
    #[inline(always)]
    pub fn new(iter: S) -> Self {
        let iter_num_channels_usize: usize = iter.num_channels() as usize;
        let iter_num_channels_f32: f32 = iter_num_channels_usize as f32;
        Self {
            iter,
            iter_num_channels_usize,
            iter_num_channels_f32,
        }
    }
}

impl<S: Source> Source for ConvertToMono<S> {}

impl<S: Source> Signal for ConvertToMono<S> {
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

impl<S: Source> Iterator for ConvertToMono<S> {
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

#[cfg(test)]
mod tests {
    use crate::backend::{Source, Waveform};

    #[test]
    fn test_size_hint_1() {
        let frame_rate_hz: u32 = 1234;
        let num_channels: u16 = 2;
        let interleaved_samples: Vec<f32> = (0..100).step_by(10).map(|x| x as f32).collect();
        let waveform = Waveform::new(frame_rate_hz, num_channels, interleaved_samples);
        let ws = waveform.to_source();
        assert_eq!(ws.size_hint().0, 10);
        assert_eq!(ws.size_hint().1.unwrap(), 10);
        let collected: Vec<usize> = ws.convert_to_mono().map(|x| x as usize).collect();
        assert_eq!(collected.len(), 5);
        assert_eq!(collected, [5, 25, 45, 65, 85]);
    }
}
