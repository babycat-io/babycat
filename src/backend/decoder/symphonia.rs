use std::io::Read;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "enable-filesystem")]
use std::convert::AsRef;
#[cfg(feature = "enable-filesystem")]
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder as SymphoniaDecoderTrait;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::formats::{FormatReader, Track};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::backend::constants::DEFAULT_FILE_EXTENSION;
use crate::backend::constants::DEFAULT_MIME_TYPE;
use crate::backend::display::est_num_frames_to_str;
use crate::backend::Error;
use crate::backend::Signal;
use crate::backend::Source;

pub struct SymphoniaDecoder {
    decoder: Box<dyn SymphoniaDecoderTrait>,
    reader: Box<dyn FormatReader>,
    frame_rate_hz: u32,
    num_channels: u16,
    num_samples_remaining: Option<usize>,
    current_packet_audio_buffer: Option<Vec<f32>>,
    current_packet_sample_idx: usize,
}

impl std::fmt::Debug for SymphoniaDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SymphoniaDecoder {{ {} frames,  {} channels,  {} hz,  {} }}",
            est_num_frames_to_str(self.num_frames_estimate()),
            self.num_channels(),
            self.frame_rate_hz(),
            self.duration_estimate_to_str(),
        )
    }
}

impl SymphoniaDecoder {
    #[allow(clippy::missing_panics_doc)]
    pub fn from_encoded_stream_with_hint<R: 'static + Read + Send + Sync>(
        encoded_stream: R,
        file_extension: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
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

        // Get the audio file's frame rate.
        let frame_rate_hz = match default_track.codec_params.sample_rate {
            None => return Err(Error::UnknownInputEncoding),
            Some(fr) => fr,
        };

        // Get the number of channels.
        // This will probably fail if Symphonia cannot recognize
        // the audio file's channel layout.
        #[allow(clippy::cast_possible_truncation)]
        let num_channels = match default_track.codec_params.channels {
            None => return Err(Error::UnknownInputEncoding),
            Some(channels) => channels.count() as u16,
        };

        #[allow(clippy::cast_possible_truncation)]
        let num_samples_remaining: Option<usize> = default_track
            .codec_params
            .n_frames
            .map(|num_frames| num_frames as usize * num_channels as usize);

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
            num_samples_remaining,
            current_packet_audio_buffer: None,
            current_packet_sample_idx: 0,
        };
        new_self.current_packet_audio_buffer = new_self.next_packet_buffer();
        Ok(new_self)
    }

    #[cfg(feature = "enable-filesystem")]
    #[allow(clippy::missing_panics_doc)]
    pub fn from_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Self, Error> {
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

    /// Returns the next packet from the Symphonia decoder.
    fn next_packet_buffer(&mut self) -> Option<Vec<f32>> {
        while let Ok(packet) = self.reader.next_packet() {
            match self.decoder.decode(&packet) {
                // Decode errors are not fatal.
                // We will just try to decode the next packet.
                Err(symphonia::core::errors::Error::DecodeError(..)) => {
                    continue;
                }

                // These errors are fatal.
                // TODO: Log this error.
                Err(_) => return None,

                Ok(decoded) => {
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as u64;
                    let mut buf = SampleBuffer::<f32>::new(duration, spec);
                    buf.copy_interleaved_ref(decoded);
                    let buf: Vec<f32> = buf.samples().to_owned();
                    return Some(buf);
                }
            }
        }
        None
    }
}

impl Source for SymphoniaDecoder {}

impl Signal for SymphoniaDecoder {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        self.num_channels
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        let num_samples_remaining = self.num_samples_remaining?;
        Some(num_samples_remaining / self.num_channels as usize)
    }
}

impl Iterator for SymphoniaDecoder {
    type Item = f32;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.num_samples_remaining {
            None => (0, None),
            Some(nsr) => (nsr, None),
        }
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let buffer = self.current_packet_audio_buffer.as_ref()?;
            if self.current_packet_sample_idx >= buffer.len() {
                self.current_packet_audio_buffer = self.next_packet_buffer();
                self.current_packet_sample_idx = 0;
                continue;
            }
            let next_sample: f32 = buffer[self.current_packet_sample_idx];
            self.num_samples_remaining =
                self.num_samples_remaining.map(|nsr| nsr.saturating_sub(1));
            self.current_packet_sample_idx += 1;
            return Some(next_sample);
        }
    }
}

#[cfg(all(test, feature = "enable-ffmpeg"))]
mod test_symphonia_source {
    use crate::decoder::SymphoniaDecoder;
    use crate::Signal;
    use crate::Source;

    const COF_FILENAME: &str = "./audio-for-tests/circus-of-freaks/track.flac";
    const COF_NUM_CHANNELS: u16 = 2;
    const COF_NUM_FRAMES: usize = 2491247;
    const COF_NUM_SAMPLES: usize = COF_NUM_FRAMES * COF_NUM_CHANNELS as usize;

    const MONO_DTMF_FILENAME: &str = "./audio-for-tests/mono-dtmf-tones/track.flac";
    const MONO_DTMF_NUM_CHANNELS: u16 = 1;
    const MONO_DTMF_NUM_FRAMES: usize = 441000;
    const MONO_DTMF_SAMPLES: usize = MONO_DTMF_NUM_FRAMES * MONO_DTMF_NUM_CHANNELS as usize;

    #[test]
    fn test_cof_size_hint_1() {
        let mut decoder =
            SymphoniaDecoder::from_file(COF_FILENAME).expect("Failed to decode circus-of-freaks");
        assert_eq!(decoder.size_hint(), (COF_NUM_SAMPLES, None));
        decoder.next();
        assert_eq!(decoder.size_hint(), (COF_NUM_SAMPLES - 1, None));
        decoder.next();
        assert_eq!(decoder.size_hint(), (COF_NUM_SAMPLES - 2, None));
        let decoder = decoder.skip(10);
        assert_eq!(decoder.size_hint(), (COF_NUM_SAMPLES - 12, None));
        let mut decoder = decoder.take(1000);
        assert_eq!(decoder.size_hint(), (1000, Some(1000)));
        decoder.next();
        assert_eq!(decoder.size_hint(), (999, Some(999)));
    }

    #[test]
    fn test_mono_dtmf_size_hint_1() {
        let mut decoder = SymphoniaDecoder::from_file(MONO_DTMF_FILENAME)
            .expect("Failed to decode mono-dtmf-tones");
        assert_eq!(decoder.size_hint(), (MONO_DTMF_SAMPLES, None));
        decoder.next();
        assert_eq!(decoder.size_hint(), (MONO_DTMF_SAMPLES - 1, None));
        decoder.next();
        assert_eq!(decoder.size_hint(), (MONO_DTMF_SAMPLES - 2, None));
        let decoder = decoder.skip(10);
        assert_eq!(decoder.size_hint(), (MONO_DTMF_SAMPLES - 12, None));
        let mut decoder = decoder.take(1000);
        assert_eq!(decoder.size_hint(), (1000, Some(1000)));
        decoder.next();
        assert_eq!(decoder.size_hint(), (999, Some(999)));
    }

    #[test]
    fn test_first_4_samples() {
        let mut decoder =
            SymphoniaDecoder::from_file("audio-for-tests/circus-of-freaks/track.flac")
                .expect("decoding error");
        let frame_rate_hz: u32 = 44100;
        let num_channels: u16 = 2;
        let num_frames_estimate: usize = 2491247;
        let num_samples_estimate: usize = num_frames_estimate * num_channels as usize;

        // Before any samples have been decoded.
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate));
        assert_eq!(decoder.size_hint(), (num_samples_estimate, None));

        // Sample 1.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 1));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 1, None));

        // Sample 2.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 1));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 2, None));

        // Sample 3.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 2));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 3, None));

        // Sample 4.
        decoder.next();
        assert_eq!(decoder.frame_rate_hz(), frame_rate_hz);
        assert_eq!(decoder.num_channels(), num_channels);
        assert_eq!(decoder.num_frames_estimate(), Some(num_frames_estimate - 2));
        assert_eq!(decoder.size_hint(), (num_samples_estimate - 4, None));

        let remaining: Vec<f32> = decoder.collect_interleaved_samples();
        assert_eq!(remaining.len(), num_samples_estimate - 4);
    }
}
