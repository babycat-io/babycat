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
mod decoder;
mod decoder_iter;
mod symphonia;

pub use crate::backend::decode::decoder::Decoder;
pub use crate::backend::decode::decoder_iter::DecoderIter;
pub use crate::backend::decode::symphonia::decoder::SymphoniaDecoder;
pub use crate::backend::decode::symphonia::decoder_iter::SymphoniaDecoderIter;

#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
mod ffmpeg;
#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub use crate::backend::decode::ffmpeg::decoder::FFmpegDecoder;
#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub use crate::backend::decode::ffmpeg::decoder_iter::FFmpegDecoderIter;
