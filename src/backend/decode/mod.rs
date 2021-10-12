//! Low-level API for decoding audio as a Rust iterator of audio samples.
//!
//! If you want to manipulate an audio waveform stored entirely in memory,
//! then you should use the [`Waveform`](crate::Waveform)
//! struct instead one of the structs implementing the
//! [`Decoder`](crate::decode::Decoder) trait.
//!
//! But if you want to decode audio as an iterator--without loading all of
//! the audio into memory at once--then you can use one of the structs
//! implementing the [`Decoder`](crate::decode::Decoder) trait.
//!
//! # Examples
//! This is how to load the entire waveform into memory, the same way that
//! [`Waveform`](crate::Waveform) does for you, using the
//! [`SymphoniaDecoder`](crate::decode::SymphoniaDecoder) audio decoder.
//!
//! ```
//! use babycat::decode::SymphoniaDecoder;
//! use babycat::decode::Decoder;
//!
//! let path = std::path::Path::new("./audio-for-tests/circus-of-freaks/track.mp3");
//! let file = std::fs::File::open(path).unwrap();
//! let mut decoder = SymphoniaDecoder::new(file, "mp3", "").unwrap();
//! let interleaved_samples: Vec<f32> = decoder.map(|x| x.unwrap()).collect();
//!
//! // We decoded 2 channels with 2491776 frames each.
//! assert_eq!(interleaved_samples.len(), 2491776 * 2);
//! ```
//!
mod symphonia;
pub use crate::backend::decode::symphonia::SymphoniaDecoder;

use std::io::Read;
use std::marker::Send;

use crate::backend::errors::Error;

/// Methods common to all audio decoders.
pub trait Decoder<T>: Iterator {
    /// Create a new audio decoder.
    fn new<R: 'static + Read + Send>(
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
