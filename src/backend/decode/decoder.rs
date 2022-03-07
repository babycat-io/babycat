use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::errors::Error;

/// Methods common to all audio decoders.
pub trait Decoder {
    fn begin(&mut self) -> Result<Box<dyn DecoderIter + '_>, Error>;

    fn frame_rate_hz(&self) -> u32;

    fn num_channels(&self) -> u16;

    fn num_frames_estimate(&self) -> Option<usize>;

    fn num_samples_estimate(&self) -> Option<usize>;
}
