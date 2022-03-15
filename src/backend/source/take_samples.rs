use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct TakeSamples<S: Source> {
    iter: S,
    count: usize,
}

impl<S: Source> TakeSamples<S> {
    #[inline(always)]
    pub fn new(iter: S, count: usize) -> Self {
        Self { iter, count }
    }
}

impl<S: Source> Source for TakeSamples<S> {}

impl<S: Source> Signal for TakeSamples<S> {
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

impl<S: Source> Iterator for TakeSamples<S> {
    type Item = f32;

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.count == 0 {
            return (0, Some(0));
        }
        let (lower, upper) = self.iter.size_hint();
        let lower = std::cmp::min(lower, self.count);
        let upper = match upper {
            Some(x) if x < self.count => Some(x),
            _ => Some(self.count),
        };
        (lower, upper)
    }

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
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

        let mut ws = ws.take_samples(5);
        assert_eq!(ws.size_hint().0, 5);
        assert_eq!(ws.size_hint().1.unwrap(), 5);

        ws.next();
        assert_eq!(ws.size_hint().0, 4);
        assert_eq!(ws.size_hint().1.unwrap(), 4);

        let ws = ws.take_samples(5);
        assert_eq!(ws.size_hint().0, 4);
        assert_eq!(ws.size_hint().1.unwrap(), 4);

        let ws = ws.take_samples(3);
        assert_eq!(ws.size_hint().0, 3);
        assert_eq!(ws.size_hint().1.unwrap(), 3);

        let ws = ws.take_samples(3);
        assert_eq!(ws.size_hint().0, 3);
        assert_eq!(ws.size_hint().1.unwrap(), 3);

        let ws = ws.take_samples(0);
        assert_eq!(ws.size_hint().0, 0);
        assert_eq!(ws.size_hint().1.unwrap(), 0);
    }
}
