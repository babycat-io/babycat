use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct SkipSamples<S: Source> {
    iter: S,
    count: usize,
}

impl<S: Source> SkipSamples<S> {
    #[inline]
    pub fn new(iter: S, count: usize) -> Self {
        Self { iter, count }
    }
}

impl<S: Source> Source for SkipSamples<S> {}

impl<S: Source> Signal for SkipSamples<S> {
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

impl<S: Source> Iterator for SkipSamples<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.count == 0 {
            return self.iter.size_hint();
        }
        let (lower, upper) = self.iter.size_hint();
        let lower = lower.saturating_sub(self.count);
        let upper = upper.map(|u| u.saturating_sub(self.count));
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.count > 0 {
            self.iter.next();
            self.count -= 1;
        }
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{Source, Waveform};

    #[test]
    fn test_size_hint_1() {
        let frame_rate_hz: u32 = 1234;
        let num_channels: u16 = 3;
        let waveform = Waveform::new(
            frame_rate_hz,
            num_channels,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        );

        let ws = waveform.to_source();
        assert_eq!(ws.size_hint().0, 6);
        assert_eq!(ws.size_hint().1.unwrap(), 6);

        let mut ws = ws.skip_samples(2);
        assert_eq!(ws.size_hint().0, 4);
        assert_eq!(ws.size_hint().1.unwrap(), 4);

        ws.next();
        assert_eq!(ws.size_hint().0, 3);
        assert_eq!(ws.size_hint().1.unwrap(), 3);
    }
}
