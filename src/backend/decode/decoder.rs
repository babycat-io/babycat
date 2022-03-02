use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use crate::backend::errors::Error;

/// Methods common to all audio decoders.
pub trait Decoder<T>: Iterator {
    /// Create a new audio decoder.
    fn new<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<Self>, Error>;

    /// The frame rate of the audio currently being decoded.
    fn frame_rate_hz(&self) -> u32;

    /// The number of channels in the audio currently being decoded.
    fn num_channels(&self) -> u32;

    /// Clean up any resources created by the decoder.
    fn close(&mut self) -> Result<(), Error>;
}
