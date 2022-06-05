use either::{Either, Either::Left, Either::Right};
use std::time::Duration;

use crate::backend::display::duration_estimate_to_str;
use crate::backend::units::frames_to_duration;

/// A trait that describes common properties of all digital audio signals.
pub trait Signal {
    /// The frame rate (or *sample rate*) of the [`Signal`], measured in frames per second.
    fn frame_rate_hz(&self) -> u32;

    /// The number of audio channels in the [`Signal`].
    fn num_channels(&self) -> u16;

    /// An estimate of the total number of frames in the [`Signal`].
    fn num_frames_estimate(&self) -> Option<usize>;

    /// The wall-clock duration of this [`Signal`], based on the estimated number of frames.
    fn duration_estimate(&self) -> Option<Duration> {
        self.num_frames_estimate()
            .map(|n| frames_to_duration(n, self.frame_rate_hz()))
    }

    /// A string representation of this [`Signal`]'s wall-clock duration.
    fn duration_estimate_to_str(&self) -> String {
        duration_estimate_to_str(self.duration_estimate())
    }
}

/// This allows us to use the [`Either`] enum for [`Signal`] objects.
impl<L, R> Signal for Either<L, R>
where
    L: Signal,
    R: Signal,
{
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        match self {
            Left(left) => left.frame_rate_hz(),
            Right(right) => right.frame_rate_hz(),
        }
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        match self {
            Left(left) => left.num_channels(),
            Right(right) => right.num_channels(),
        }
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        match self {
            Left(left) => left.num_frames_estimate(),
            Right(right) => right.num_frames_estimate(),
        }
    }

    #[inline]
    fn duration_estimate(&self) -> Option<Duration> {
        match self {
            Left(left) => left.duration_estimate(),
            Right(right) => right.duration_estimate(),
        }
    }

    #[inline]
    fn duration_estimate_to_str(&self) -> String {
        match self {
            Left(left) => left.duration_estimate_to_str(),
            Right(right) => right.duration_estimate_to_str(),
        }
    }
}
