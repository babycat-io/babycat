use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatReader, Track};

use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::errors::Error;

pub struct SymphoniaDecoderIter<'a> {
    decoder: Box<dyn SymphoniaDecoderTrait>,
    reader: &'a mut Box<dyn FormatReader>,
    current_packet_audio_buffer: Option<SampleBuffer<f32>>,
    current_packet_sample_idx: usize,
    error: Result<(), Error>,
}

impl<'a> SymphoniaDecoderIter<'a> {
    pub fn new(reader: &'a mut Box<dyn FormatReader>) -> Result<Self, Error> {
        let decoder_opts: DecoderOptions = DecoderOptions { verify: false };
        let default_track: &Track = match reader.default_track() {
            None => return Err(Error::NoSuitableAudioStreams(reader.tracks().len())),
            Some(dt) => dt,
        };
        let _num_channels: usize = default_track.codec_params.channels.unwrap().count();
        let decoder = match symphonia::default::get_codecs()
            .make(&default_track.codec_params, &decoder_opts)
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
        let mut new_self = Self {
            decoder,
            reader,
            current_packet_audio_buffer: None,
            current_packet_sample_idx: 0,
            error: Ok(()),
        };
        new_self.current_packet_audio_buffer = new_self.next_packet_buffer();
        Ok(new_self)
    }
    /// Returns the next packet from the Symphonia decoder.
    fn next_packet_buffer(&mut self) -> Option<SampleBuffer<f32>> {
        while let Ok(packet) = self.reader.next_packet() {
            match self.decoder.decode(&packet) {
                // Decode errors are not fatal.
                // We will just try to decode the next packet.
                Err(symphonia::core::errors::Error::DecodeError(..)) => {
                    continue;
                }

                Err(_) => {
                    self.error = Err(Error::UnknownDecodeError);
                    return None;
                }

                Ok(decoded) => {
                    let spec = decoded.spec().to_owned();
                    let duration = decoded.capacity() as u64;
                    let mut buffer = SampleBuffer::<f32>::new(duration, spec);
                    buffer.copy_interleaved_ref(decoded);
                    return Some(buffer);
                }
            }
        }
        None
    }
}

impl<'a> DecoderIter for SymphoniaDecoderIter<'a> {}

impl<'a> Iterator for SymphoniaDecoderIter<'a> {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let buffer = self.current_packet_audio_buffer.as_ref()?;
            if self.current_packet_sample_idx >= buffer.len() {
                self.current_packet_audio_buffer = self.next_packet_buffer();
                self.current_packet_sample_idx = 0;
                continue;
            }
            let next_sample = buffer.samples()[self.current_packet_sample_idx];
            self.current_packet_sample_idx += 1;
            return Some(next_sample);
        }
    }
}
