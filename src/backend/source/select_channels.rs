use crate::backend::display::est_num_frames_to_str;
use crate::backend::Signal;
use crate::backend::Source;

/// [`Source::select_first_channels()`]
pub struct SelectChannels<S: Source> {
    iter: S,
    original_num_channels: usize,
    selected_num_channels: usize,
    channel_idx: usize,
}

impl<S: Source> SelectChannels<S> {
    #[inline]
    pub fn new(iter: S, selected_num_channels: u16) -> Self {
        let original_num_channels: usize = iter.num_channels() as usize;
        let selected_num_channels: usize =
            std::cmp::min(selected_num_channels as usize, original_num_channels);
        Self {
            iter,
            original_num_channels,
            selected_num_channels,
            channel_idx: 0,
        }
    }
}

impl<S: Source> std::fmt::Debug for SelectChannels<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SelectChannels {{ {} frames,  {} -> {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.original_num_channels,
            self.selected_num_channels,
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl<S: Source> Source for SelectChannels<S> {}

impl<S: Source> Signal for SelectChannels<S> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.iter.frame_rate_hz()
    }

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn num_channels(&self) -> u16 {
        self.selected_num_channels as u16
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.iter.num_frames_estimate()
    }
}

impl<S: Source> Iterator for SelectChannels<S> {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        let lower = (lower * self.selected_num_channels + self.original_num_channels - 1)
            / self.original_num_channels;
        let upper = upper.map(|u| {
            (u * self.selected_num_channels + self.original_num_channels - 1)
                / self.original_num_channels
        });
        (lower, upper)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iter_next = self.iter.next();
            let channel_idx = self.channel_idx;
            self.channel_idx = (self.channel_idx + 1) % self.original_num_channels;
            if channel_idx >= self.selected_num_channels {
                continue;
            }
            return iter_next;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{Source, Waveform};

    #[test]
    fn test_size_hint_1() {
        let frame_rate_hz: u32 = 1234;
        let num_channels: u16 = 5;
        let samples: Vec<usize> = (0..11).collect();
        let waveform = Waveform::new(
            frame_rate_hz,
            num_channels,
            samples.iter().map(|x| *x as f32).collect(),
        );

        let ws = waveform.into_source();
        assert_eq!(ws.size_hint().0, 11);
        assert_eq!(ws.size_hint().1.unwrap(), 11);

        let ws = ws.select_first_channels(3);
        assert_eq!(ws.size_hint().0, 7);
        assert_eq!(ws.size_hint().1.unwrap(), 7);
        let collected: Vec<usize> = ws.map(|x| x as usize).collect();
        assert_eq!(collected.len(), 7);
        assert_eq!(collected, [0, 1, 2, 5, 6, 7, 10]);
    }
}
