//! Functions for converting from one unit of measurement to another.
use std::time::Duration;

#[inline]
pub fn frames_to_samples(num_frames: usize, num_channels: u16) -> usize {
    num_frames * (num_channels as usize)
}

#[inline]
pub fn samples_to_frames(num_samples: usize, num_channels: u16) -> usize {
    num_samples / (num_channels as usize)
}

#[inline]
pub fn milliseconds_to_frames(duration_milliseconds: usize, frame_rate_hz: u32) -> usize {
    (duration_milliseconds * frame_rate_hz as usize) / 1000
}

#[inline]
pub fn milliseconds_to_duration(duration_milliseconds: usize) -> Duration {
    Duration::from_millis(duration_milliseconds as u64)
}

#[inline]
pub fn frames_to_duration(num_frames: usize, frame_rate_hz: u32) -> Duration {
    let ms = frames_to_milliseconds(num_frames, frame_rate_hz);
    milliseconds_to_duration(ms)
}

#[inline]
pub fn milliseconds_to_samples(
    duration_milliseconds: usize,
    frame_rate_hz: u32,
    num_channels: u16,
) -> usize {
    let num_frames = milliseconds_to_frames(duration_milliseconds, frame_rate_hz);
    frames_to_samples(num_frames, num_channels)
}

///
/// # Examples
/// ```
/// use babycat::units::frames_to_milliseconds;
///
/// let num_frames = 1323000;
/// let frame_rate_hz = 44100;
/// let thirty_seconds_as_ms = 30 * 1000;
/// assert_eq!(
///     frames_to_milliseconds(num_frames, frame_rate_hz),
///     thirty_seconds_as_ms
/// );
/// ```
#[inline]
pub fn frames_to_milliseconds(num_frames: usize, frame_rate_hz: u32) -> usize {
    num_frames * 1000 / frame_rate_hz as usize
}

#[inline]
pub fn samples_to_milliseconds(num_samples: usize, frame_rate_hz: u32, num_channels: u16) -> usize {
    let num_frames = samples_to_frames(num_samples, num_channels);
    frames_to_milliseconds(num_frames, frame_rate_hz)
}
