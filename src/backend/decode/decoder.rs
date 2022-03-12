use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::errors::Error;
use crate::backend::signal::Signal;

/// Methods common to all audio decoders.
pub trait Decoder: Signal {
    fn begin(&mut self) -> Result<Box<dyn DecoderIter + '_>, Error>;
}

impl Decoder for Box<dyn Decoder> {
    #[inline(always)]
    fn begin(&mut self) -> Result<Box<dyn DecoderIter + '_>, Error> {
        (&mut **self).begin()
    }
}

impl Signal for Box<dyn Decoder> {
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
