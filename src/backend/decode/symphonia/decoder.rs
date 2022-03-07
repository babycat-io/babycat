use std::convert::AsRef;
use std::io::Read;
use std::marker::Send;
use std::marker::Sync;
use std::path::Path;

use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::{FormatReader, Track};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::decode::decoder::Decoder;
use crate::backend::errors::Error;
use crate::backend::waveform_args::DEFAULT_FILE_EXTENSION;
use crate::backend::waveform_args::DEFAULT_MIME_TYPE;

use crate::backend::decode::symphonia::decoder_iter::SymphoniaDecoderIter;

/// An audio decoder from Philip Deljanov's [`symphonia`] audio decoding library.
pub struct SymphoniaDecoder {
    reader: Box<dyn FormatReader>,
    frame_rate: u32,
    channels: u16,
    est_num_frames: Option<usize>,
}

impl SymphoniaDecoder {
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<dyn Decoder>, Error> {
        // Set up defaults for the decoder.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();

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
        let frame_rate = default_track.codec_params.sample_rate.unwrap();
        let channels = default_track.codec_params.channels.unwrap().count() as u16;

        let est_num_frames: Option<usize> = default_track
            .codec_params
            .n_frames
            .map(|n_frames| n_frames as usize);

        Ok(Box::new(Self {
            reader,
            frame_rate,
            channels,
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
    fn begin(&mut self) -> Result<Box<dyn Iterator<Item = f32> + '_>, Error> {
        let decode_iter = SymphoniaDecoderIter::new(&mut self.reader)?;
        Ok(Box::new(decode_iter))
    }
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.channels
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.est_num_frames
    }
}
