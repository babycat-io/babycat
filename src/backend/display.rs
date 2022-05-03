//! Utilities for rendering objects as strings.

use std::time::Duration;

use humantime::format_duration;

/// Serialize an estimate of a number of frames to [`String`].
#[inline]
pub fn est_num_frames_to_str(est_num_frames: Option<usize>) -> String {
    match est_num_frames {
        None => "unknown".to_string(),
        Some(f) => format!("{}", f),
    }
}

/// Serialize a [`std::time::Duration`] to [`String`].
#[inline]
pub fn duration_to_str(d: Duration) -> String {
    format_duration(d).to_string()
}

/// Serialize a duration estimate to [`String`].
#[inline]
pub fn duration_estimate_to_str(duration_estimate: Option<Duration>) -> String {
    match duration_estimate {
        None => "unknown duration".to_string(),
        Some(d) => duration_to_str(d),
    }
}
