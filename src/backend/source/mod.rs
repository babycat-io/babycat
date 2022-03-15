mod convert_to_mono;
mod select_channels;
mod skip_samples;
mod take_samples;
mod waveform_source;

pub use convert_to_mono::ConvertToMono;
pub use select_channels::SelectChannels;
pub use skip_samples::SkipSamples;
pub use take_samples::TakeSamples;
pub use waveform_source::WaveformSource;

use crate::backend::signal::Signal;

/// A sample iterator created by an audio decoder.
pub trait Source: Signal + Iterator<Item = f32> {
    #[inline(always)]
    fn skip_samples(self, count: usize) -> SkipSamples<Self>
    where
        Self: Sized,
    {
        SkipSamples::new(self, count)
    }

    #[inline(always)]
    fn take_samples(self, count: usize) -> TakeSamples<Self>
    where
        Self: Sized,
    {
        TakeSamples::new(self, count)
    }

    #[inline(always)]
    fn select_first_channels(self, selected_num_channels: u16) -> SelectChannels<Self>
    where
        Self: Sized,
    {
        SelectChannels::new(self, selected_num_channels)
    }

    #[inline(always)]
    fn convert_to_mono(self) -> ConvertToMono<Self>
    where
        Self: Sized,
    {
        ConvertToMono::new(self)
    }
}

impl Source for Box<dyn Source + '_> {}

impl Signal for Box<dyn Source + '_> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        (&**self).frame_rate_hz()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        (&**self).num_channels()
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        (&**self).num_frames_estimate()
    }
}
