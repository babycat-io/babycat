use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatReader, Track};

use crate::backend::errors::Error;
use crate::backend::signal::Signal;
use crate::backend::Source;

pub struct SymphoniaSource<'a> {
    decoder: Box<dyn SymphoniaDecoderTrait>,
    reader: &'a mut Box<dyn FormatReader>,
    frame_rate_hz: u32,
    num_channels: u16,
    est_num_frames: Option<usize>,
    sample_idx: usize,
    current_packet_audio_buffer: Option<SampleBuffer<f32>>,
    current_packet_sample_idx: usize,
    error: Result<(), Error>,
}

impl<'a> SymphoniaSource<'a> {
    pub fn new(
        reader: &'a mut Box<dyn FormatReader>,
        frame_rate_hz: u32,
        num_channels: u16,
        est_num_frames: Option<usize>,
    ) -> Result<Self, Error> {
        let decoder_opts: DecoderOptions = DecoderOptions { verify: false };
        let default_track: &Track = match reader.default_track() {
            None => return Err(Error::NoSuitableAudioStreams(reader.tracks().len())),
            Some(dt) => dt,
        };
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
            frame_rate_hz,
            num_channels,
            est_num_frames,
            sample_idx: 0,
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

impl<'a> Source for SymphoniaSource<'a> {}

impl<'a> Signal for SymphoniaSource<'a> {
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
        match self.est_num_frames {
            None => None,
            Some(est_num_frames) => {
                let current_frame_idx = self.sample_idx / self.num_channels as usize;
                if current_frame_idx >= est_num_frames {
                    return None;
                }
                Some(est_num_frames - current_frame_idx)
            }
        }
    }
}

impl<'a> Iterator for SymphoniaSource<'a> {
    type Item = f32;

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.est_num_frames {
            None => (0, None),
            Some(est_num_frames) => {
                let est_num_samples = est_num_frames * self.num_channels as usize;
                if self.sample_idx >= est_num_samples {
                    return (0, None);
                }
                (est_num_samples - self.sample_idx, None)
            }
        }
    }

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
            self.sample_idx += 1;
            self.current_packet_sample_idx += 1;
            return Some(next_sample);
        }
    }
}

#[cfg(all(test, feature = "enable-ffmpeg"))]
mod test_symphonia_source {
    use crate::backend::symphonia::SymphoniaDecoder;

    pub const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.flac";
    pub const COF_NUM_CHANNELS: u16 = 2;
    pub const COF_NUM_FRAMES: usize = 2491247;
    pub const COF_NUM_SAMPLES: usize = COF_NUM_FRAMES * COF_NUM_CHANNELS as usize;

    pub const MONO_DTMF_FILENAME: &str = "./audio-for-tests/mono-dtmf-tones/track.flac";
    pub const MONO_DTMF_NUM_CHANNELS: u16 = 1;
    pub const MONO_DTMF_NUM_FRAMES: usize = 441000;
    pub const MONO_DTMF_SAMPLES: usize = MONO_DTMF_NUM_FRAMES * MONO_DTMF_NUM_CHANNELS as usize;

    #[test]
    fn test_cof_size_hint_1() {
        let mut decoder =
            SymphoniaDecoder::from_file(COF_FILENAME).expect("Failed to decode circus-of-freaks");
        let mut source = decoder.begin().expect("Failed to create Source");
        assert_eq!(source.size_hint(), (COF_NUM_SAMPLES, None));
        source.next();
        assert_eq!(source.size_hint(), (COF_NUM_SAMPLES - 1, None));
        source.next();
        assert_eq!(source.size_hint(), (COF_NUM_SAMPLES - 2, None));
        let source = source.skip(10);
        assert_eq!(source.size_hint(), (COF_NUM_SAMPLES - 12, None));
        let mut source = source.take(1000);
        assert_eq!(source.size_hint(), (1000, Some(1000)));
        source.next();
        assert_eq!(source.size_hint(), (999, Some(999)));
    }

    #[test]
    fn test_mono_dtmf_size_hint_1() {
        let mut decoder = SymphoniaDecoder::from_file(MONO_DTMF_FILENAME)
            .expect("Failed to decode mono-dtmf-tones");
        let mut source = decoder.begin().expect("Failed to create Source");
        assert_eq!(source.size_hint(), (MONO_DTMF_SAMPLES, None));
        source.next();
        assert_eq!(source.size_hint(), (MONO_DTMF_SAMPLES - 1, None));
        source.next();
        assert_eq!(source.size_hint(), (MONO_DTMF_SAMPLES - 2, None));
        let source = source.skip(10);
        assert_eq!(source.size_hint(), (MONO_DTMF_SAMPLES - 12, None));
        let mut source = source.take(1000);
        assert_eq!(source.size_hint(), (1000, Some(1000)));
        source.next();
        assert_eq!(source.size_hint(), (999, Some(999)));
    }
}
