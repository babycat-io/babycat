use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatReader, Track};

use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::decode_args::DecodeArgs;
use crate::backend::errors::Error;

pub struct SymphoniaDecoderIter<'a> {
    args: DecodeArgs,
    decoder: Box<dyn SymphoniaDecoderTrait>,
    reader: &'a mut Box<dyn FormatReader>,
    select_first_channels: usize,
    current_packet_audio_buffer: Option<SampleBuffer<f32>>,
    current_packet_sample_idx: usize,
    error: Result<(), Error>,
}

impl<'a> DecoderIter for SymphoniaDecoderIter<'a> {}

impl<'a> SymphoniaDecoderIter<'a> {
    pub fn new(args: DecodeArgs, reader: &'a mut Box<dyn FormatReader>) -> Result<Self, Error> {
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
            args,
            decoder,
            reader,
            select_first_channels: args.num_channels as usize,
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

    #[inline(always)]
    fn next_multichannel(&mut self) -> Option<f32> {
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

    #[inline(always)]
    fn next_mono(&mut self) -> Option<f32> {
        let mut psum: f32 = 0.0_f32;
        for _i in 0..self.select_first_channels {
            match self.next_multichannel() {
                None => return None,
                Some(val) => psum += val,
            }
        }
        Some(psum / self.select_first_channels as f32)
    }
}

impl<'a> Iterator for SymphoniaDecoderIter<'a> {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.args.convert_to_mono {
            return self.next_mono();
        }
        self.next_multichannel()
    }
}
