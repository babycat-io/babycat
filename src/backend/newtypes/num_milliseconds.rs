use std::fmt;
use std::ops;
use std::time::Duration;

use crate::backend::newtypes::FrameRateHz;
use crate::backend::newtypes::NumChannels;
use crate::backend::newtypes::NumFrames;
use crate::backend::newtypes::NumSamples;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumMilliseconds(pub usize);

impl fmt::Display for NumMilliseconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NumMilliseconds {
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }

    pub fn into_duration(self) -> Duration {
        Duration::from_millis(self.into_usize() as u64)
    }

    pub fn into_num_frames(self, frame_rate_hz: FrameRateHz) -> NumFrames {
        NumFrames(self.into_usize() * frame_rate_hz.into_usize() / 1000)
    }

    pub fn into_num_samples(
        self,
        frame_rate_hz: FrameRateHz,
        num_channels: NumChannels,
    ) -> NumSamples {
        NumSamples(
            self.into_usize() * frame_rate_hz.into_usize() * num_channels.into_usize() / 1000,
        )
    }
}

impl From<usize> for NumMilliseconds {
    fn from(num: usize) -> NumMilliseconds {
        NumMilliseconds(num)
    }
}

impl From<Duration> for NumMilliseconds {
    fn from(duration: Duration) -> NumMilliseconds {
        NumMilliseconds(duration.as_millis() as usize)
    }
}

impl ops::Add<NumMilliseconds> for NumMilliseconds {
    type Output = NumMilliseconds;

    fn add(self, rhs: NumMilliseconds) -> Self::Output {
        NumMilliseconds(self.into_usize() + rhs.into_usize())
    }
}

impl ops::Sub<NumMilliseconds> for NumMilliseconds {
    type Output = NumMilliseconds;

    fn sub(self, rhs: NumMilliseconds) -> Self::Output {
        NumMilliseconds(self.into_usize() - rhs.into_usize())
    }
}
