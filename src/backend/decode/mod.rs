pub mod symphonia;

use std::io::Read;
use std::marker::Send;

use crate::backend::errors::Error;

pub trait Decoder<T>: Iterator {
    fn new<R: 'static + Read + Send>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<Self>, Error>;

    fn frame_rate_hz(&self) -> u32;

    fn num_channels(&self) -> u32;

    fn close(&mut self) -> Result<(), Error>;
}
