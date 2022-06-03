//! Iterators that decode encoded audio files or
//! streams into a [`Source`] yielding `f32` samples.
//!
//! # Examples
//! ## Using [`SymphoniaDecoder`]
//!
//! ```
//! // Decoders typically require importing the Source and Signal traits.
//! use babycat::{Source, Signal};
//! use babycat::decoder::SymphoniaDecoder;
//!
//! // Begin decoding an audio file.
//! let filename = "audio-for-tests/circus-of-freaks/track.flac";
//! let mut decoder = SymphoniaDecoder::from_file(filename).expect("decoding error");
//!
//! // Get properties about the audio.
//! assert_eq!(decoder.frame_rate_hz(), 44100);
//! assert_eq!(decoder.num_channels(), 2);
//! assert_eq!(decoder.num_frames_estimate(), Some(2491247));
//!
//! // Decoders are iterators. Let's get the first two audio samples.
//! let sample1: Option<f32> = decoder.next();
//! let sample2: Option<f32> = decoder.next();
//!
//! // Iterate over the decoder, storing all of the samples in-memory, except
//! // the above two we iterated over.
//! let waveform: babycat::Waveform = decoder.to_waveform();
//! ```
//!
//! ## Using the default decoder
//! Some decoding backends, like [`FFmpegDecoder`], can be included or
//! excluded at compile-time via Cargo features. If you do not know
//! which audio decoders will be available to you, you can request
//! the "default" decoding backend:
//! ```
//! use babycat::{Source, Signal};
//! use babycat::decoder;
//!
//! let filename = "audio-for-tests/circus-of-freaks/track.flac";
//! let d = decoder::from_file(filename).expect("decoding failed");
//! assert_eq!(d.num_frames_estimate(), Some(2491247));
//! ```
//!
#![allow(dead_code)]

#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
mod ffmpeg;

mod symphonia;

#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub use ffmpeg::FFmpegDecoder;

pub use self::symphonia::SymphoniaDecoder;

use std::convert::AsRef;
use std::io::Cursor;
use std::io::Read;
use std::marker::Send;
use std::marker::Sync;
use std::path::Path;

use crate::backend::constants;
use crate::backend::Error;
use crate::backend::Source;

type DecoderResult = Result<Box<dyn Source>, Error>;

pub fn from_encoded_stream_with_hint_by_backend<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
    file_extension: &str,
    mime_type: &str,
) -> DecoderResult {
    match decoding_backend {
        constants::DEFAULT_DECODING_BACKEND | constants::DECODING_BACKEND_SYMPHONIA => {
            Ok(Box::new(SymphoniaDecoder::from_encoded_stream_with_hint(
                encoded_stream,
                file_extension,
                mime_type,
            )?))
        }
        _ => Err(Error::FeatureNotCompiled("unknown-decoding-backend")),
    }
}

pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
    encoded_stream: R,
    file_extension: &str,
    mime_type: &str,
) -> DecoderResult {
    from_encoded_stream_with_hint_by_backend(
        constants::DEFAULT_DECODING_BACKEND,
        encoded_stream,
        file_extension,
        mime_type,
    )
}

pub fn from_encoded_stream_by_backend<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
) -> DecoderResult {
    from_encoded_stream_with_hint_by_backend(
        decoding_backend,
        encoded_stream,
        constants::DEFAULT_FILE_EXTENSION,
        constants::DEFAULT_MIME_TYPE,
    )
}

#[inline]
pub fn from_encoded_stream<R: 'static + Read + Send + Sync>(encoded_stream: R) -> DecoderResult {
    from_encoded_stream_by_backend(constants::DEFAULT_DECODING_BACKEND, encoded_stream)
}

#[inline]
pub fn from_encoded_bytes_with_hint_by_backend(
    decoding_backend: u32,
    encoded_bytes: &[u8],
    file_extension: &str,
    mime_type: &str,
) -> DecoderResult {
    let owned = encoded_bytes.to_owned();
    let encoded_stream = Cursor::new(owned);
    from_encoded_stream_with_hint_by_backend(
        decoding_backend,
        encoded_stream,
        file_extension,
        mime_type,
    )
}

#[inline]
pub fn from_encoded_bytes_with_hint(
    encoded_bytes: &[u8],
    file_extension: &str,
    mime_type: &str,
) -> DecoderResult {
    from_encoded_bytes_with_hint_by_backend(
        constants::DEFAULT_DECODING_BACKEND,
        encoded_bytes,
        file_extension,
        mime_type,
    )
}

#[inline]
pub fn from_encoded_bytes_by_backend(decoding_backend: u32, encoded_bytes: &[u8]) -> DecoderResult {
    from_encoded_bytes_with_hint_by_backend(
        decoding_backend,
        encoded_bytes,
        constants::DEFAULT_FILE_EXTENSION,
        constants::DEFAULT_MIME_TYPE,
    )
}

#[inline]
pub fn from_encoded_bytes(encoded_bytes: &[u8]) -> DecoderResult {
    from_encoded_bytes_with_hint_by_backend(
        constants::DEFAULT_DECODING_BACKEND,
        encoded_bytes,
        constants::DEFAULT_FILE_EXTENSION,
        constants::DEFAULT_MIME_TYPE,
    )
}

#[cfg(feature = "enable-filesystem")]
pub fn from_file_by_backend<F: Clone + AsRef<Path>>(
    decoding_backend: u32,
    filename: F,
) -> DecoderResult {
    #[allow(clippy::match_same_arms)]
    match decoding_backend {
        constants::DEFAULT_DECODING_BACKEND => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                Ok(Box::new(FFmpegDecoder::from_file(filename)?))
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                Ok(Box::new(SymphoniaDecoder::from_file(filename)?))
            }
        }
        constants::DECODING_BACKEND_SYMPHONIA => {
            Ok(Box::new(SymphoniaDecoder::from_file(filename)?))
        }
        constants::DECODING_BACKEND_FFMPEG => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                Ok(Box::new(FFmpegDecoder::from_file(filename)?))
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                Err(Error::FeatureNotCompiled("ffmpeg"))
            }
        }
        _ => Err(Error::FeatureNotCompiled("unknown-decoding-backend")),
    }
}

#[cfg(feature = "enable-filesystem")]
#[inline]
pub fn from_file<F: Clone + AsRef<Path>>(filename: F) -> DecoderResult {
    from_file_by_backend(constants::DEFAULT_DECODING_BACKEND, filename)
}
