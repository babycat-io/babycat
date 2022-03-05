use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use symphonia::core::codecs::CodecParameters;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::FormatReader;
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::common::get_est_num_frames;
use crate::backend::decode::decoder::Decoder;
use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::decode_args::DecodeArgs;
use crate::backend::errors::Error;
use crate::backend::waveform_args::WaveformArgs;
use crate::backend::waveform_args::DEFAULT_FILE_EXTENSION;
use crate::backend::waveform_args::DEFAULT_MIME_TYPE;

use crate::backend::decode::symphonia::decoder_iter::SymphoniaDecoderIter;

/// An audio decoder from Philip Deljanov's [`symphonia`] audio decoding library.
pub struct SymphoniaDecoder {
    args: DecodeArgs,
    reader: Box<dyn FormatReader>,
    codec_params: CodecParameters,
    frame_rate: u32,
    channels: u16,
    est_num_frames: usize,
}

impl SymphoniaDecoder {
    pub fn new<R: 'static + Read + Send + Sync>(
        waveform_args: WaveformArgs,
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
        let mut reader = probed.format;
        let codec_params = reader.default_track().unwrap().codec_params;

        // Examine the actual shape of this audio file.
        let frame_rate = codec_params.sample_rate.unwrap();
        let channels = codec_params.channels.unwrap().count() as u16;

        let args = DecodeArgs::new(waveform_args, frame_rate, channels)?;

        let est_num_frames = match codec_params.n_frames {
            Some(n_frames) => {
                get_est_num_frames(n_frames as usize, args.start_frame_idx, args.end_frame_idx)
            }
            None => get_est_num_frames(usize::MAX, args.start_frame_idx, args.end_frame_idx),
        };

        Ok(Box::new(Self {
            args,
            reader,
            codec_params,
            frame_rate,
            channels,
            est_num_frames,
        }))
    }
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
        waveform_args: WaveformArgs,
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<dyn Decoder>, Error> {
        Self::new(waveform_args, encoded_stream, file_extension, mime_type)
    }

    #[cfg(feature = "enable-filesystem")]
    pub fn from_file(
        filename: &str,
        waveform_args: WaveformArgs,
    ) -> Result<Box<dyn Decoder>, Error> {
        let pathname = std::path::Path::new(filename);
        let file = match std::fs::File::open(pathname) {
            Ok(f) => f,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(Error::FileNotFound(Box::leak(
                        filename.to_owned().into_boxed_str(),
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
                    filename.to_owned().into_boxed_str(),
                )));
            }
        }
        let file_extension = match pathname.extension() {
            Some(os_str) => match os_str.to_str() {
                Some(str) => str,
                None => DEFAULT_FILE_EXTENSION,
            },
            None => DEFAULT_FILE_EXTENSION,
        };

        Self::from_encoded_stream_with_hint(waveform_args, file, file_extension, DEFAULT_MIME_TYPE)
    }
}

impl Decoder for SymphoniaDecoder {
    #[inline(always)]
    fn begin(mut self) -> Result<Box<dyn DecoderIter>, Error> {
        match SymphoniaDecoderIter::new(self.args, &mut self.reader, &self.codec_params) {
            Ok(decoder) => Ok(Box::new(decoder)),
            Err(error) => Err(error),
        }
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
        Some(self.est_num_frames)
    }

    #[inline(always)]
    fn num_samples_estimate(&self) -> Option<usize> {
        Some(self.est_num_frames * self.channels as usize)
    }
}
