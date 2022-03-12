use std::fmt::{Debug, Display};

pub trait Sample: Copy + Debug + Display + Sized + PartialOrd + PartialEq {
    fn as_f32_sample(self) -> f32;
}

const I16_DENOM: f32 = ((i16::MAX as i32) + 1_i32) as f32;

impl Sample for i16 {
    #[inline(always)]
    fn as_f32_sample(self) -> f32 {
        (self as f32) / I16_DENOM
    }
}

const I32_DENOM: f64 = ((i32::MAX as i64) + 1_i64) as f64;

impl Sample for i32 {
    #[inline(always)]
    fn as_f32_sample(self) -> f32 {
        ((self as f64) / I32_DENOM) as f32
    }
}

impl Sample for f32 {
    #[inline(always)]
    fn as_f32_sample(self) -> f32 {
        self
    }
}

impl Sample for f64 {
    #[inline(always)]
    fn as_f32_sample(self) -> f32 {
        self as f32
    }
}
