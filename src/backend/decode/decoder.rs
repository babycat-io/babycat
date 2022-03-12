use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::errors::Error;
use crate::backend::signal::Signal;

/// Methods common to all audio decoders.
pub trait Decoder: Signal {
    fn begin(&mut self) -> Result<Box<dyn DecoderIter + '_>, Error>;
}
