use std::fmt;
use std::ops;

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumChannels(pub u16);

impl fmt::Display for NumChannels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NumChannels {
    pub fn into_u16(self) -> u16 {
        self.0 as u16
    }

    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u16> for NumChannels {
    fn from(num: u16) -> NumChannels {
        NumChannels(num)
    }
}

impl From<NumChannels> for u16 {
    fn from(num_channels: NumChannels) -> u16 {
        num_channels.into_u16()
    }
}

impl From<NumChannels> for usize {
    fn from(num_channels: NumChannels) -> usize {
        num_channels.into_usize()
    }
}

impl ops::Add<NumChannels> for NumChannels {
    type Output = NumChannels;

    fn add(self, rhs: NumChannels) -> Self::Output {
        NumChannels(self.into_u16() + rhs.into_u16())
    }
}

impl ops::Sub<NumChannels> for NumChannels {
    type Output = NumChannels;

    fn sub(self, rhs: NumChannels) -> Self::Output {
        NumChannels(self.into_u16() - rhs.into_u16())
    }
}
