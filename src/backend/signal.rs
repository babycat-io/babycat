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
