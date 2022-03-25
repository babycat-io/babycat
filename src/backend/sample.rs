use std::fmt::{Debug, Display};

pub trait Sample: Copy + Debug + Display + Sized + PartialOrd + PartialEq {
    fn as_f32_sample(self) -> f32;
}

#[allow(clippy::cast_precision_loss)]
const I16_DENOM: f32 = ((i16::MAX as i32) + 1_i32) as f32;

impl Sample for i16 {
    #[inline]
    fn as_f32_sample(self) -> f32 {
        f32::from(self) / I16_DENOM
    }
}

#[allow(clippy::cast_precision_loss)]
const I32_DENOM: f64 = ((i32::MAX as i64) + 1_i64) as f64;

impl Sample for i32 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_f32_sample(self) -> f32 {
        (f64::from(self) / I32_DENOM) as f32
    }
}

impl Sample for f32 {
    #[inline]
    fn as_f32_sample(self) -> f32 {
        self
    }
}

impl Sample for f64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_f32_sample(self) -> f32 {
        self as f32
    }
}
