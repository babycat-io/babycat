use std::convert::AsRef;
use std::io::Read;
use std::marker::Send;
use std::marker::Sync;
use std::path::Path;

use crate::backend::decode::decoder::Decoder;

use crate::backend::errors::Error;
use crate::backend::waveform_args::DECODING_BACKEND_FFMPEG;
use crate::backend::waveform_args::DECODING_BACKEND_SYMPHONIA;
use crate::backend::waveform_args::DEFAULT_DECODING_BACKEND;
use crate::backend::waveform_args::DEFAULT_FILE_EXTENSION;
use crate::backend::waveform_args::DEFAULT_MIME_TYPE;

pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
    file_extension: &str,
    mime_type: &str,
) -> Result<Box<dyn Decoder>, Error> {
    match decoding_backend {
        DEFAULT_DECODING_BACKEND | DECODING_BACKEND_SYMPHONIA => {
            crate::backend::decode::symphonia::decoder::SymphoniaDecoder::from_encoded_stream_with_hint(
                encoded_stream,
                file_extension,
                mime_type,
            )
        }
        _ => Err(Error::FeatureNotCompiled("decoding-backend-1")),
    }
}

pub fn from_encoded_stream<R: 'static + Read + Send + Sync>(
    decoding_backend: u32,
    encoded_stream: R,
) -> Result<Box<dyn Decoder>, Error> {
    from_encoded_stream_with_hint(
        decoding_backend,
        encoded_stream,
        DEFAULT_FILE_EXTENSION,
        DEFAULT_MIME_TYPE,
    )
}

pub fn from_encoded_bytes_with_hint(
    decoding_backend: u32,
    encoded_bytes: &[u8],
    file_extension: &str,
    mime_type: &str,
) -> Result<Box<dyn Decoder>, Error> {
    let owned = encoded_bytes.to_owned();
    let encoded_stream = std::io::Cursor::new(owned);
    from_encoded_stream_with_hint(decoding_backend, encoded_stream, file_extension, mime_type)
}

pub fn from_encoded_bytes(
    decoding_backend: u32,
    encoded_bytes: &[u8],
) -> Result<Box<dyn Decoder>, Error> {
    from_encoded_bytes_with_hint(
        decoding_backend,
        encoded_bytes,
        DEFAULT_FILE_EXTENSION,
        DEFAULT_MIME_TYPE,
    )
}

#[cfg(feature = "enable-filesystem")]
pub fn from_file<F: Clone + AsRef<Path>>(
    decoding_backend: u32,
    filename: F,
) -> Result<Box<dyn Decoder>, Error> {
    match decoding_backend {
        DEFAULT_DECODING_BACKEND => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                crate::backend::decode::ffmpeg::decoder::FFmpegDecoder::from_file(filename)
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                crate::backend::decode::symphonia::decoder::SymphoniaDecoder::from_file(filename)
            }
        }
        DECODING_BACKEND_SYMPHONIA => {
            crate::backend::decode::symphonia::decoder::SymphoniaDecoder::from_file(filename)
        }
        DECODING_BACKEND_FFMPEG => {
            #[cfg(feature = "enable-ffmpeg")]
            {
                crate::backend::decode::ffmpeg::decoder::FFmpegDecoder::from_file(filename)
            }
            #[cfg(not(feature = "enable-ffmpeg"))]
            {
                Err(Error::FeatureNotCompiled("decoding-backend-2"))
            }
        }
        _ => Err(Error::FeatureNotCompiled("decoding-backend-3")),
    }
}
