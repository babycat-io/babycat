use std::convert::AsRef;
use std::path::Path;

use ffmpeg::decoder::Audio as AudioDecoder;
use ffmpeg::format::context::Input;
use ffmpeg::util::format::sample::Sample as FFmpegSample;
use ffmpeg::Stream;

use crate::backend::decode::ffmpeg::ffmpeg_init;

use crate::backend::common::get_est_num_frames;
use crate::backend::decode::decoder::Decoder;
use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::decode::ffmpeg::decoder_iter::FFmpegDecoderIter;
use crate::backend::decode_args::DecodeArgs;
use crate::backend::errors::Error;
use crate::backend::waveform_args::WaveformArgs;

#[inline(always)]
fn new_input_for_file<F: Clone + AsRef<Path>>(filename: F) -> Result<Input, Error> {
    ffmpeg::format::input(&filename).map_err(|_| Error::UnknownDecodeError)
}

#[inline(always)]
fn get_first_working_audio_stream(input: &Input) -> Result<(Stream, AudioDecoder), Error> {
    let mut num_found_streams = 0;
    for input_stream in input.streams() {
        num_found_streams += 1;
        match ffmpeg::codec::context::Context::from_parameters(input_stream.parameters()) {
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

#[inline(always)]
fn estimate_num_frames_inner(
    stream_duration: i64,
    stream_tb_n: i32,
    stream_tb_d: i32,
    decoder_tb_n: i32,
    decoder_tb_d: i32,
) -> usize {
    let mut x = stream_duration as f64;
    x *= decoder_tb_d as f64;
    x *= stream_tb_n as f64;
    x /= stream_tb_d as f64;
    x /= decoder_tb_n as f64;
    x = x.ceil();
    x as usize
}

#[inline(always)]
pub fn estimate_num_frames(stream: &ffmpeg::Stream, decoder: &ffmpeg::decoder::Audio) -> usize {
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
    est_num_frames: usize,
    args: DecodeArgs,
}

impl FFmpegDecoder {
    fn new(
        input: Input,
        decoder: AudioDecoder,
        stream_index: usize,
        original_est_num_frames: usize,
        waveform_args: WaveformArgs,
    ) -> Result<Box<dyn Decoder>, Error> {
        match decoder.format() {
            FFmpegSample::None | FFmpegSample::U8(_) | FFmpegSample::I64(_) => {
                return Err(Error::UnknownDecodeError);
            }
            _ => (),
        };
        let args = DecodeArgs::new(waveform_args, decoder.rate(), decoder.channels())?;
        let est_num_frames = get_est_num_frames(
            original_est_num_frames,
            args.start_frame_idx,
            args.end_frame_idx,
        );
        Ok(Box::new(Self {
            args,
            input,
            decoder,
            stream_index,
            est_num_frames,
        }))
    }
    pub fn from_file<F: Clone + AsRef<Path>>(
        filename: F,
        waveform_args: WaveformArgs,
    ) -> Result<Box<dyn Decoder>, Error> {
        ffmpeg_init();
        let input = new_input_for_file(filename)?;
        let (stream, decoder) = get_first_working_audio_stream(&input)?;
        let original_est_num_frames = estimate_num_frames(&stream, &decoder);
        let stream_index = stream.index();
        Self::new(
            input,
            decoder,
            stream_index,
            original_est_num_frames,
            waveform_args,
        )
    }
}

impl Decoder for FFmpegDecoder {
    #[inline(always)]
    fn begin(self) -> Result<Box<dyn DecoderIter>, Error> {
        Ok(Box::new(FFmpegDecoderIter::new(
            &mut self.input,
            &mut self.decoder,
            self.stream_index,
            self.args,
        )))
    }

    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.decoder.rate()
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.args.num_channels
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        Some(self.est_num_frames)
    }

    #[inline(always)]
    fn num_samples_estimate(&self) -> Option<usize> {
        Some(self.est_num_frames * self.args.num_channels as usize)
    }
}
