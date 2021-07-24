use std::io::Read;
use std::marker::Send;

use symphonia::core::audio::AudioBuffer;
use symphonia::core::audio::AudioBufferRef;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::FormatReader;
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::decode::Decoder;
use crate::backend::decode_args::*;
use crate::backend::errors::Error;
use crate::backend::sample_rescaling::i32_to_f32;

pub enum AudioBufferType {
    F32(AudioBuffer<f32>),
    S32(AudioBuffer<i32>),
}

pub struct SymphoniaDecoderF32 {
    _decoder: Box<dyn SymphoniaDecoderTrait>,
    _reader: Box<dyn FormatReader>,
    _current_packet_audio_buffer: Option<AudioBufferType>,
    _current_packet_frame_idx: usize,
    _current_packet_channel_idx: usize,
    _frame_rate_hz: u32,
    _num_channels: u32,
}

impl SymphoniaDecoderF32 {
    fn next_packet_buffer(&mut self) -> Option<Result<AudioBufferType, Error>> {
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

                Ok(decoded_buffer_ref) => match decoded_buffer_ref {
                    AudioBufferRef::F32(buf_f32) => {
                        return Some(Ok(AudioBufferType::F32(buf_f32.into_owned())));
                    }
                    AudioBufferRef::S32(buf_i32) => {
                        return Some(Ok(AudioBufferType::S32(buf_i32.into_owned())));
                    }
                },
            }
        }
        None
    }
}

impl Decoder<f32> for SymphoniaDecoderF32 {
    fn new<R: 'static + Read + Send>(
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
            hint.with_extension(&file_extension);
        }
        if mime_type != DEFAULT_MIME_TYPE {
            hint.mime_type(&mime_type);
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
            _current_packet_frame_idx: 0,
            _current_packet_channel_idx: 0,
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
        self._decoder.close();
        Ok(())
    }
}

impl Iterator for SymphoniaDecoderF32 {
    type Item = Result<f32, Error>;

    fn next(&mut self) -> Option<Self::Item> {
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
                        self._current_packet_frame_idx = 0;
                        self._current_packet_channel_idx = 0;
                    }
                },
            }
        }
        let (next_sample, packet_size) = match self._current_packet_audio_buffer.as_ref().unwrap() {
            AudioBufferType::F32(buf_f32) => {
                let current_channel = buf_f32.chan(self._current_packet_channel_idx);
                let ns = current_channel[self._current_packet_frame_idx];
                (ns, current_channel.len())
            }
            AudioBufferType::S32(buf_i32) => {
                let current_channel = buf_i32.chan(self._current_packet_channel_idx);
                let ns = i32_to_f32(current_channel[self._current_packet_frame_idx]);
                (ns, current_channel.len())
            }
        };

        // Bump the channel index. Next time, we will query
        // the sample belonging to the NEXT channel in the SAME frame...
        self._current_packet_channel_idx =
            (self._current_packet_channel_idx + 1) % self._num_channels as usize;

        // ... unless the channel index is zero, in which case we are
        // now in the NEXT frame...
        if self._current_packet_channel_idx == 0 {
            self._current_packet_frame_idx += 1;
        }

        // ...unless the frame index is the same as the packet size,
        // which means that we are now in the NEXT packet.
        if self._current_packet_frame_idx >= packet_size {
            self._current_packet_audio_buffer = None;
        }

        Some(Ok(next_sample))
    }
}
