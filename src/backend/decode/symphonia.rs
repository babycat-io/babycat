use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::FormatReader;
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::decode::decoder::Decoder;
use crate::backend::errors::Error;
use crate::backend::waveform_args::DEFAULT_FILE_EXTENSION;
use crate::backend::waveform_args::DEFAULT_MIME_TYPE;

/// An audio decoder from Philip Deljanov's [`symphonia`] audio decoding library.
pub struct SymphoniaDecoder {
    _decoder: Box<dyn SymphoniaDecoderTrait>,
    _reader: Box<dyn FormatReader>,
    _current_packet_audio_buffer: Option<SampleBuffer<f32>>,
    _current_packet_sample_idx: usize,
    _frame_rate_hz: u32,
    _num_channels: u32,
}

impl SymphoniaDecoder {
    /// Returns the next packet from the Symphonia decoder.
    fn next_packet_buffer(&mut self) -> Option<Result<SampleBuffer<f32>, Error>> {
        while let Ok(packet) = self._reader.next_packet() {
            match self._decoder.decode(&packet) {
                // Decode errors are not fatal.
                // We will just try to decode the next packet.
                Err(symphonia::core::errors::Error::DecodeError(..)) => {
                    continue;
                }

                Err(_) => {
                    let _ = self.close();
                    return Some(Err(Error::UnknownDecodeError));
                }

                Ok(decoded) => {
                    let spec = decoded.spec().to_owned();
                    let duration = decoded.capacity() as u64;
                    let mut buffer = SampleBuffer::<f32>::new(duration, spec);
                    buffer.copy_interleaved_ref(decoded);
                    return Some(Ok(buffer));
                }
            }
        }
        None
    }
}

impl Decoder<f32> for SymphoniaDecoder {
    fn new<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Box<Self>, Error> {
        // Set up defaults for the decoder.
        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = DecoderOptions { verify: false };

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
        let mut _reader = probed.format;
        let track = _reader.default_track().unwrap();
        let codec_params = &track.codec_params;
        let mut _decoder = match symphonia::default::get_codecs().make(codec_params, &decoder_opts)
        {
            Ok(value) => value,
            // If we could not identify the input as one of our supported
            // encodings, then throw an error.
            Err(symphonia::core::errors::Error::Unsupported { .. }) => {
                return Err(Error::UnknownInputEncoding);
            }
            // Raise unknown errors.
            Err(_) => {
                return Err(Error::UnknownDecodeError);
            }
        };

        // Examine the actual shape of this audio file.
        let _frame_rate_hz = codec_params.sample_rate.unwrap();
        let _num_channels = codec_params.channels.unwrap().count() as u32;

        Ok(Box::new(Self {
            _decoder,
            _reader,
            _current_packet_audio_buffer: None,
            _current_packet_sample_idx: 0,
            _frame_rate_hz,
            _num_channels,
        }))
    }

    fn frame_rate_hz(&self) -> u32 {
        self._frame_rate_hz
    }

    fn num_channels(&self) -> u32 {
        self._num_channels
    }

    fn close(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl Iterator for SymphoniaDecoder {
    type Item = Result<f32, Error>;

    /// Returns the next interleaved audio sample.
    fn next(&mut self) -> Option<Self::Item> {
        // If the audio buffer is None, then fill the buffer with the next audio packet.
        if self._current_packet_audio_buffer.is_none() {
            let audio_buffer_opt_result = self.next_packet_buffer();
            match audio_buffer_opt_result {
                // If there is no next packet to decode, then we have reached
                // the end of our audio stream.
                None => {
                    return None;
                }
                Some(audio_buffer_opt_result) => match audio_buffer_opt_result {
                    // If we encountered an error when decoding the next packet,
                    // then pass the error to the caller.
                    Err(error) => {
                        return Some(Err(error));
                    }
                    // We have successfully decoded the next packet. Update our
                    // struct private variables.
                    Ok(audio_buffer) => {
                        self._current_packet_audio_buffer = Some(audio_buffer);
                        self._current_packet_sample_idx = 0;
                    }
                },
            }
        }
        // Get the audio buffer.
        let current_packet_audio_buffer = self._current_packet_audio_buffer.as_ref().unwrap();

        // Look up the next sample in the buffer.
        let next_sample = current_packet_audio_buffer.samples()[self._current_packet_sample_idx];
        self._current_packet_sample_idx += 1;

        // ...unless the frame index is the same as the packet size,
        // which means that we are now in the NEXT packet.
        if self._current_packet_sample_idx >= current_packet_audio_buffer.len() {
            self._current_packet_audio_buffer = None;
        }

        Some(Ok(next_sample))
    }
}
