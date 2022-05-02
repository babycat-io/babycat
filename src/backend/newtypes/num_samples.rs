use std::fmt;
use std::ops;

use crate::backend::newtypes::NumChannels;
use crate::backend::newtypes::NumFrames;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumSamples(pub usize);

impl fmt::Display for NumSamples {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NumSamples {
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for NumSamples {
    fn from(num: usize) -> NumSamples {
        NumSamples(num)
    }
}

impl From<NumSamples> for usize {
    fn from(num_samples: NumSamples) -> usize {
        num_samples.into_usize()
    }
}

impl ops::Div<NumChannels> for NumSamples {
    type Output = NumFrames;

    fn div(self, rhs: NumChannels) -> Self::Output {
        NumFrames(self.into_usize() / rhs.into_usize())
    }
}

impl ops::Add<NumSamples> for NumSamples {
    type Output = NumSamples;

    fn add(self, rhs: NumSamples) -> Self::Output {
        NumSamples(self.into_usize() + rhs.into_usize())
    }
}

impl ops::Sub<NumSamples> for NumSamples {
    type Output = NumSamples;

    fn sub(self, rhs: NumSamples) -> Self::Output {
        NumSamples(self.into_usize() - rhs.into_usize())
    }
}
