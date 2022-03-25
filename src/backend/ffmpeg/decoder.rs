use std::convert::AsRef;
use std::path::Path;

use ffmpeg_next::decoder::Audio as AudioDecoder;
use ffmpeg_next::format::context::Input;
use ffmpeg_next::util::error::Error as FFmpegError;
use ffmpeg_next::util::error::ENOENT;
use ffmpeg_next::util::format::sample::Sample as FFmpegSample;
use ffmpeg_next::util::format::sample::Type::Packed;
use ffmpeg_next::util::format::sample::Type::Planar;
use ffmpeg_next::Stream;

use crate::backend::errors::Error;
use crate::backend::ffmpeg::ffmpeg_init;
use crate::backend::ffmpeg::FFmpegSource;
use crate::backend::signal::Signal;
use crate::backend::Decoder;
use crate::backend::Source;

#[inline]
fn new_input_for_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Input, Error> {
    let filename_ref = filename.as_ref();
    ffmpeg_next::format::input(&filename_ref).map_err(|err| match err {
        // File not found error. Audio filename was not found on the local filesystem.
        FFmpegError::Other { errno: ENOENT } => Error::FileNotFound(Box::leak(
            filename_ref.to_str().unwrap().to_owned().into_boxed_str(),
        )),
        _ => Error::UnknownDecodeError,
    })
}

#[inline]
fn get_first_working_audio_stream(input: &Input) -> Result<(Stream, AudioDecoder), Error> {
    let mut num_found_streams = 0;
    for input_stream in input.streams() {
        num_found_streams += 1;
        match ffmpeg_next::codec::context::Context::from_parameters(input_stream.parameters()) {
            Err(_) => continue,
            Ok(codec_context) => match codec_context.decoder().audio() {
                Err(_) => continue,
                Ok(mut decoder) => {
                    if decoder.set_parameters(input_stream.parameters()).is_err() {
                        continue;
                    }
                    return Ok((input_stream, decoder));
                }
            },
        }
    }
    Err(Error::NoSuitableAudioStreams(num_found_streams))
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn estimate_num_frames_inner(
    stream_duration: i64,
    stream_tb_n: i32,
    stream_tb_d: i32,
    decoder_tb_n: i32,
    decoder_tb_d: i32,
) -> usize {
    #[allow(clippy::cast_precision_loss)]
    let mut x = stream_duration as f64;
    x *= f64::from(decoder_tb_d);
    x *= f64::from(stream_tb_n);
    x /= f64::from(stream_tb_d);
    x /= f64::from(decoder_tb_n);
    x = x.ceil();
    x as usize
}

#[inline]
pub fn estimate_num_frames(
    stream: &ffmpeg_next::Stream,
    decoder: &ffmpeg_next::decoder::Audio,
) -> usize {
    let stream_duration = stream.duration();
    let stream_tb = stream.time_base();
    let decoder_tb = decoder.time_base();
    estimate_num_frames_inner(
        stream_duration,
        stream_tb.0,
        stream_tb.1,
        decoder_tb.0,
        decoder_tb.1,
    )
}

pub struct FFmpegDecoder {
    input: Input,
    decoder: AudioDecoder,
    stream_index: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    est_num_frames: Option<usize>,
}

impl FFmpegDecoder {
    #[allow(clippy::new_ret_no_self)]
    fn new(
        input: Input,
        decoder: AudioDecoder,
        stream_index: usize,
        est_num_frames: Option<usize>,
    ) -> Result<Box<dyn Decoder>, Error> {
        match decoder.format() {
            FFmpegSample::None | FFmpegSample::U8(_) | FFmpegSample::I64(_) => {
                return Err(Error::UnknownDecodeError);
            }
            _ => (),
        };
        let frame_rate_hz: u32 = decoder.rate();
        let num_channels: u16 = decoder.channels();
        Ok(Box::new(Self {
            input,
            decoder,
            stream_index,
            frame_rate_hz,
            num_channels,
            est_num_frames,
        }))
    }
    pub fn from_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Box<dyn Decoder>, Error> {
        ffmpeg_init();
        let input = new_input_for_file(filename)?;
        let (stream, decoder) = get_first_working_audio_stream(&input)?;
        let est_num_frames = Some(estimate_num_frames(&stream, &decoder));
        let stream_index = stream.index();
        Self::new(input, decoder, stream_index, est_num_frames)
    }
}

impl Decoder for FFmpegDecoder {
    #[inline]
    fn begin(&mut self) -> Result<Box<dyn Source + '_>, Error> {
        let format = self.decoder.format();
        match format {
            FFmpegSample::I16(Packed) => Ok(Box::new(FFmpegSource::<i16, true>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::I16(Planar) => Ok(Box::new(FFmpegSource::<i16, false>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::I32(Packed) => Ok(Box::new(FFmpegSource::<i32, true>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::I32(Planar) => Ok(Box::new(FFmpegSource::<i32, false>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::F32(Packed) => Ok(Box::new(FFmpegSource::<f32, true>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::F32(Planar) => Ok(Box::new(FFmpegSource::<f32, false>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::F64(Packed) => Ok(Box::new(FFmpegSource::<f64, true>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            FFmpegSample::F64(Planar) => Ok(Box::new(FFmpegSource::<f64, false>::new(
                &mut self.input,
                &mut self.decoder,
                self.stream_index,
                self.frame_rate_hz,
                self.num_channels,
                self.est_num_frames,
            ))),
            _ => panic!("NO"),
        }
    }
}

impl Signal for FFmpegDecoder {
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
        self.est_num_frames
    }
}
