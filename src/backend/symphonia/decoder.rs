use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "enable-filesystem")]
use std::convert::AsRef;
#[cfg(feature = "enable-filesystem")]
use std::path::Path;

use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::{FormatReader, Track};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::constants::DEFAULT_FILE_EXTENSION;
use crate::backend::constants::DEFAULT_MIME_TYPE;
use crate::backend::errors::Error;
use crate::backend::signal::Signal;
use crate::backend::Decoder;
use crate::backend::DecoderIter;

use crate::backend::symphonia::SymphoniaDecoderIter;

/// An audio decoder from Philip Deljanov's [`symphonia`] audio decoding library.
pub struct SymphoniaDecoder {
    reader: Box<dyn FormatReader>,
    frame_rate_hz: u32,
    num_channels: u16,
    est_num_frames: Option<usize>,
}

impl SymphoniaDecoder {
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<dyn Decoder>, Error> {
        // Set up defaults for the decoder.
        let metadata_opts: MetadataOptions = Default::default();

        // We enable "gapless playback" in Symphonia so it will not give
        // us the useless/empty frames at the beginning/end of
        // an MP3 stream.
        let format_opts: FormatOptions = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };

        // Provide file extension and mime type hints to speed up
        // guessing which audio format the input is.
        // An incorrect hint will not prevent a successful decoding.
        let mut hint = Hint::new();
        if file_extension != DEFAULT_FILE_EXTENSION {
            hint.with_extension(file_extension);
        }
        if mime_type != DEFAULT_MIME_TYPE {
            hint.mime_type(mime_type);
        }

        // Initialize the decoder.
        let media_source: Box<dyn MediaSource> = Box::new(ReadOnlySource::new(encoded_stream));
        let media_source_stream = MediaSourceStream::new(media_source, Default::default());
        let probed = match symphonia::default::get_probe().format(
            &hint,
            media_source_stream,
            &format_opts,
            &metadata_opts,
        ) {
            Ok(value) => value,
            // If we could not identify the input as one of our supported
            // encodings, then throw an error.
            Err(symphonia::core::errors::Error::Unsupported { .. }) => {
                return Err(Error::UnknownInputEncoding);
            }
            // Raise unknown errors.
            Err(err) => {
                return Err(Error::UnknownDecodeErrorWithMessage(leak_str!(
                    err.to_string()
                )))
            }
        };
        let reader = probed.format;
        let default_track: &Track = match reader.default_track() {
            None => return Err(Error::NoSuitableAudioStreams(reader.tracks().len())),
            Some(dt) => dt,
        };

        // Examine the actual shape of this audio file.
        let frame_rate_hz = default_track.codec_params.sample_rate.unwrap();
        let num_channels = default_track.codec_params.channels.unwrap().count() as u16;

        let est_num_frames: Option<usize> = default_track
            .codec_params
            .n_frames
            .map(|n_frames| n_frames as usize);

        Ok(Box::new(Self {
            reader,
            frame_rate_hz,
            num_channels,
            est_num_frames,
        }))
    }

    #[cfg(feature = "enable-filesystem")]
    pub fn from_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Box<dyn Decoder>, Error> {
        let filename_ref = filename.as_ref();
        let file = match std::fs::File::open(filename_ref) {
            Ok(f) => f,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(Error::FileNotFound(Box::leak(
                        filename_ref.to_str().unwrap().to_owned().into_boxed_str(),
                    )));
                }
                _ => {
                    return Err(Error::UnknownIOError);
                }
            },
        };
        if let Ok(metadata) = file.metadata() {
            if metadata.is_dir() {
                return Err(Error::FilenameIsADirectory(Box::leak(
                    filename_ref.to_str().unwrap().to_owned().into_boxed_str(),
                )));
            }
        }
        let file_extension = match filename.as_ref().extension() {
            Some(os_str) => match os_str.to_str() {
                Some(str) => str,
                None => DEFAULT_FILE_EXTENSION,
            },
            None => DEFAULT_FILE_EXTENSION,
        };

        Self::from_encoded_stream_with_hint(file, file_extension, DEFAULT_MIME_TYPE)
    }
}

impl Decoder for SymphoniaDecoder {
    #[inline(always)]
    fn begin(&mut self) -> Result<Box<dyn DecoderIter + '_>, Error> {
        let decode_iter = SymphoniaDecoderIter::new(
            &mut self.reader,
            self.frame_rate_hz,
            self.num_channels,
            self.est_num_frames,
        )?;
        Ok(Box::new(decode_iter))
    }
}

impl Signal for SymphoniaDecoder {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.num_channels
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.est_num_frames
    }
}
