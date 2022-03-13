use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct SelectChannels<S: Source> {
    iter: S,
    original_num_channels: usize,
    selected_num_channels: usize,
    disabled: bool,
    channel_idx: usize,
}

impl<S: Source> SelectChannels<S> {
    #[inline]
    pub fn new(iter: S, selected_num_channels: u16) -> Self {
        let original_num_channels: usize = iter.num_channels() as usize;
        let selected_num_channels: usize =
            std::cmp::min(selected_num_channels as usize, original_num_channels);
        let disabled: bool =
            selected_num_channels == 0 || selected_num_channels == original_num_channels;
        Self {
            iter,
            original_num_channels,
            selected_num_channels,
            disabled,
            channel_idx: 0,
        }
    }
}

impl<S: Source> Source for SelectChannels<S> {}

impl<S: Source> Signal for SelectChannels<S> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.iter.frame_rate_hz()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.selected_num_channels as u16
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.iter.num_frames_estimate()
    }
}

impl<S: Source> Iterator for SelectChannels<S> {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.disabled {
            return self.iter.next();
        }
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
