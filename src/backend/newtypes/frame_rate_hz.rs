use std::fmt;
use std::ops;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameRateHz(pub u32);

impl fmt::Display for FrameRateHz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FrameRateHz {
    pub fn into_u32(self) -> u32 {
        self.0 as u32
    }

    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for FrameRateHz {
    fn from(num: u32) -> FrameRateHz {
        FrameRateHz(num)
    }
}

impl ops::Add<FrameRateHz> for FrameRateHz {
    type Output = FrameRateHz;

    fn add(self, rhs: FrameRateHz) -> Self::Output {
        FrameRateHz(self.into_u32() + rhs.into_u32())
    }
}

impl ops::Sub<FrameRateHz> for FrameRateHz {
    type Output = FrameRateHz;

    fn sub(self, rhs: FrameRateHz) -> Self::Output {
        FrameRateHz(self.into_u32() - rhs.into_u32())
    }
}

impl ops::Mul<f64> for FrameRateHz {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        f64::from(self.into_u32()) * rhs
    }
}

impl ops::Mul<f32> for FrameRateHz {
    type Output = f32;

    #[allow(clippy::cast_precision_loss)]
    fn mul(self, rhs: f32) -> Self::Output {
        self.into_u32() as f32 * rhs
    }
}

impl ops::Div<f64> for FrameRateHz {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        f64::from(self.into_u32()) / rhs
    }
}

impl ops::Div<f32> for FrameRateHz {
    type Output = f32;

    #[allow(clippy::cast_precision_loss)]
    fn div(self, rhs: f32) -> Self::Output {
        self.into_u32() as f32 / rhs
    }
}

impl ops::Div<FrameRateHz> for FrameRateHz {
    type Output = f64;

    #[allow(clippy::cast_precision_loss)]
    fn div(self, rhs: FrameRateHz) -> Self::Output {
        f64::from(self.into_u32()) / f64::from(rhs.into_u32())
    }
}
