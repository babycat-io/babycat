use std::fmt;

use crate::backend::waveform::Waveform;
use serde::{Deserialize, Serialize};

/// Represents a fixed-length audio waveform as a `Vec<i16>`.
///
/// This struct should only be used when needed to store or transmit
/// uncompressed audio data. For more operations on audio, it is better
/// to work with a [FloatWaveform](crate::FloatWaveform).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntWaveform {
    interleaved_samples: Vec<i16>,
    frame_rate_hz: u32,
    num_channels: u32,
    num_frames: u64,
}

impl From<crate::backend::float_waveform::FloatWaveform> for IntWaveform {
    fn from(item: crate::backend::float_waveform::FloatWaveform) -> Self {
        let buffer: Vec<i16> = item
            .interleaved_samples()
            .iter()
            .map(|val| ((*val) * 0x8000 as f32) as i16)
            .collect();

        IntWaveform {
            interleaved_samples: buffer,
            frame_rate_hz: item.frame_rate_hz(),
            num_channels: item.num_channels(),
            num_frames: item.num_frames(),
        }
    }
}

// We manually implement the debug trait so that we don't
// print out giant vectors.
impl fmt::Debug for IntWaveform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IntWaveform {{ frame_rate_hz: {}, num_channels: {}, num_frames: {}}}",
            self.frame_rate_hz(),
            self.num_channels(),
            self.num_frames()
        )
    }
}

impl crate::backend::waveform::Waveform for IntWaveform {
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    fn num_channels(&self) -> u32 {
        self.num_channels
    }

    fn num_frames(&self) -> u64 {
        self.num_frames
    }
}

impl IntWaveform {
    pub fn new(frame_rate_hz: u32, num_channels: u32, interleaved_samples: Vec<i16>) -> Self {
        let num_frames = interleaved_samples.len() as u64 / num_channels as u64;
        IntWaveform {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            num_frames,
        }
    }

    pub fn interleaved_samples(&self) -> &[i16] {
        &self.interleaved_samples
    }
}
