use ffmpeg::decoder::Audio as AudioDecoder;
use ffmpeg::frame::Audio as Frame;
use ffmpeg::util::format::sample::Sample as FFmpegSample;
use ffmpeg::util::format::sample::Type::Packed;
use ffmpeg::util::format::sample::Type::Planar;

use crate::backend::decode::ffmpeg::sample::Sample;

pub struct FrameIter {
    frame: Frame,
    format: FFmpegSample,
    num_channels: usize,
    num_frames: usize,
    channel_idx: usize,
    frame_idx: usize,
}

impl FrameIter {
    pub fn new(decoder: &mut AudioDecoder) -> Option<Self> {
        let mut frame = Frame::empty();
        decoder.receive_frame(&mut frame).ok()?;
        let num_channels = frame.channels() as usize;
        let num_frames: usize = frame.samples();
        let format = frame.format();
        Some(Self {
            frame,
            format,
            num_channels,
            num_frames,
            channel_idx: 0,
            frame_idx: 0,
        })
    }

    #[inline(always)]
    fn get_sample_packed<T: Sample>(&self) -> f32 {
        let sample_idx: usize = self.frame_idx * self.num_channels + self.channel_idx;
        unsafe {
            // When audio is in a "packed" format, FFmpeg stores
            // each sample interleaved in the first data plane.
            let plane_ptr: *const T = (*self.frame.as_ptr()).data[0] as *const T;
            let sample: T = *plane_ptr.add(sample_idx);
            sample.as_f32_sample()
        }
    }

    #[inline(always)]
    fn get_sample_planar<T: Sample>(&self) -> f32 {
        unsafe {
            // When audio is stored in a "planar" format, FFmpeg
            // stores the first eight planes in the `.data` attribute.
            // If there are more than 8 planes, all of them are
            // available in the `.extended_data` attribute.
            // If there are not more than 8 planes, then
            // `.extended_data` just points to `.data`.
            let extended_data_ptr: *const *const T =
                (*self.frame.as_ptr()).extended_data as *const *const T;
            let plane_ptr: *const T = *extended_data_ptr.add(self.channel_idx);
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
}

impl Iterator for FrameIter {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.channel_idx >= self.num_channels {
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
}
