use crate::backend::signal::Signal;

use crate::backend::decode::convert_to_mono_decoder_iter::ConvertToMonoDecoderIter;
use crate::backend::decode::select_channels_decoder_iter::SelectChannelsDecoderIter;
use crate::backend::decode::skip_samples_decoder_iter::SkipSamplesDecoderIter;
use crate::backend::decode::take_samples_decoder_iter::TakeSamplesDecoderIter;

/// A sample iterator created by an audio decoder.
pub trait DecoderIter: Signal + Iterator<Item = f32> {
    #[inline(always)]
    fn skip_samples(self, count: usize) -> SkipSamplesDecoderIter<Self>
    where
        Self: Sized,
    {
        SkipSamplesDecoderIter::new(self, count)
    }

    #[inline(always)]
    fn take_samples(self, count: usize) -> TakeSamplesDecoderIter<Self>
    where
        Self: Sized,
    {
        TakeSamplesDecoderIter::new(self, count)
    }

    #[inline(always)]
    fn select_first_channels(self, selected_num_channels: u16) -> SelectChannelsDecoderIter<Self>
    where
        Self: Sized,
    {
        SelectChannelsDecoderIter::new(self, selected_num_channels)
    }

    #[inline(always)]
    fn convert_to_mono(self, enabled: bool) -> ConvertToMonoDecoderIter<Self>
    where
        Self: Sized,
    {
        ConvertToMonoDecoderIter::new(self, enabled)
    }
}

impl DecoderIter for Box<dyn DecoderIter + '_> {}

impl Signal for Box<dyn DecoderIter + '_> {
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
