use ffmpeg::decoder::Audio as AudioDecoder;
use ffmpeg::frame::Audio as Frame;
use ffmpeg::util::format::sample::Sample as FFmpegSample;
use ffmpeg::util::format::sample::Type::Packed;
use ffmpeg::util::format::sample::Type::Planar;

use crate::backend::decode::ffmpeg::sample::Sample;
use crate::backend::decode_args::DecodeArgs;

pub struct FrameIter {
    frame: Frame,
    args: DecodeArgs,
    format: FFmpegSample,
    num_channels: usize,
    num_frames: usize,
    select_first_channels: usize,
    channel_idx: usize,
    frame_idx: usize,
}

impl FrameIter {
    pub fn new(decoder: &mut AudioDecoder, args: DecodeArgs) -> Option<Self> {
        let mut frame = Frame::empty();
        decoder.receive_frame(&mut frame).ok()?;
        let num_channels = frame.channels() as usize;
        let select_first_channels: usize = args.num_channels as usize;
        let num_frames: usize = frame.samples();
        let format = frame.format();
        Some(Self {
            frame,
            args,
            format,
            num_channels,
            num_frames,
            select_first_channels,
            channel_idx: 0,
            frame_idx: 0,
        })
    }

    #[inline(always)]
    fn get_sample_packed<T: Sample>(&self) -> f32 {
        let sample_idx: usize = self.frame_idx * self.num_channels + self.channel_idx;
        unsafe {
            let plane_ptr: *const T = (*self.frame.as_ptr()).data[0] as *const T;
            let sample: T = *plane_ptr.add(sample_idx);
            sample.as_f32_sample()
        }
    }

    #[inline(always)]
    fn get_sample_planar<T: Sample>(&self) -> f32 {
        unsafe {
            let plane_ptr: *const T = (*self.frame.as_ptr()).data[self.channel_idx] as *const T;
            let sample: T = *plane_ptr.add(self.frame_idx);
            sample.as_f32_sample()
        }
    }

    #[inline(always)]
    fn get_sample(&self) -> f32 {
        match self.format {
            FFmpegSample::I16(Packed) => self.get_sample_packed::<i16>(),
            FFmpegSample::I16(Planar) => self.get_sample_planar::<i16>(),
            FFmpegSample::I32(Packed) => self.get_sample_packed::<i32>(),
            FFmpegSample::I32(Planar) => self.get_sample_planar::<i32>(),
            FFmpegSample::F32(Packed) => self.get_sample_packed::<f32>(),
            FFmpegSample::F32(Planar) => self.get_sample_planar::<f32>(),
            FFmpegSample::F64(Packed) => self.get_sample_packed::<f64>(),
            FFmpegSample::F64(Planar) => self.get_sample_planar::<f64>(),
            _ => panic!("NO"),
        }
    }

    #[inline(always)]
    fn next_multichannel(&mut self) -> Option<f32> {
        if self.channel_idx >= self.select_first_channels {
            self.channel_idx = 0;
            self.frame_idx += 1;
        }
        if self.frame_idx >= self.num_frames {
            return None;
        }
        let retval = self.get_sample();
        self.channel_idx += 1;
        Some(retval)
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

impl Iterator for FrameIter {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.args.convert_to_mono {
            return self.next_mono();
        }
        self.next_multichannel()
    }
}
