use std::fmt;
use std::ops;

use crate::backend::newtypes::NumChannels;
use crate::backend::newtypes::NumSamples;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumFrames(pub usize);

impl fmt::Display for NumFrames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NumFrames {
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }

    pub fn into_num_samples(self, num_channels: NumChannels) -> NumSamples {
        self * num_channels
    }
}

impl PartialEq<usize> for NumFrames {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl From<usize> for NumFrames {
    fn from(num: usize) -> NumFrames {
        NumFrames(num)
    }
}

impl From<NumFrames> for usize {
    fn from(num_frames: NumFrames) -> usize {
        num_frames.into_usize()
    }
}

impl ops::Mul<NumChannels> for NumFrames {
    type Output = NumSamples;

    fn mul(self, rhs: NumChannels) -> Self::Output {
        NumSamples(self.into_usize() * rhs.into_usize())
    }
}

impl ops::Add<NumFrames> for NumFrames {
    type Output = NumFrames;

    fn add(self, rhs: NumFrames) -> Self::Output {
        NumFrames(self.into_usize() + rhs.into_usize())
    }
}

impl ops::Sub<NumFrames> for NumFrames {
    type Output = NumFrames;

    fn sub(self, rhs: NumFrames) -> Self::Output {
        NumFrames(self.into_usize() - rhs.into_usize())
    }
}
