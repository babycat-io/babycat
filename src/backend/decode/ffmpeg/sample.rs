use ffmpeg::util::frame::audio::Sample as FFmpegSampleTrait;
use std::fmt::{Debug, Display};

pub trait Sample:
    Copy + Debug + Display + Sized + PartialOrd + PartialEq + FFmpegSampleTrait
{
    fn as_f32_sample(self) -> f32;
}

const I16_DENOM: f64 = (i16::MAX as f64) + 1.0_f64;

impl Sample for i16 {
    fn as_f32_sample(self) -> f32 {
        ((self as f64) / I16_DENOM) as f32
    }
}

const I32_DENOM: f64 = (i32::MAX as f64) + 1.0_f64;

impl Sample for i32 {
    fn as_f32_sample(self) -> f32 {
        ((self as f64) / I32_DENOM) as f32
    }
}

impl Sample for f32 {
    fn as_f32_sample(self) -> f32 {
        self
    }
}

impl Sample for f64 {
    fn as_f32_sample(self) -> f32 {
        self as f32
    }
}
