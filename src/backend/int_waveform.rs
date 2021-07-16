use std::fmt;

use crate::backend::sample_rescaling::f32_to_i16;
use crate::backend::waveform::Waveform;
use serde::{Deserialize, Serialize};

/// Represents a fixed-length audio waveform as a `Vec<i16>`.
///
/// This struct should only be used when needed to store or transmit
/// uncompressed audio data. For more operations on audio, it is better
/// to work with a [`FloatWaveform`](crate::FloatWaveform).
///
/// # Examples
/// ```
/// use babycat::{FloatWaveform, IntWaveform, Waveform};
///
///
/// // The FloatWaveform stores audio samples as f32 values between -1.0 and 1.0.
/// let float_waveform = FloatWaveform::from_file(
///    "audio-for-tests/circus-of-freaks/track.mp3",
///     Default::default(),
/// ).unwrap();
/// assert_eq!(
///     format!("{:?}", float_waveform),
///     "FloatWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2492928}"
/// );
/// println!("{:?}", &float_waveform.interleaved_samples()[30000..30005]);
/// // [0.0238994, 0.08098572, 0.0208567, 0.09139156, 0.015145444]
///
///
/// // The Intwaveform stores audio samples as i16 values between -32768 and 32767.
/// let int_waveform = IntWaveform::from(float_waveform);
/// assert_eq!(
///     format!("{:?}", int_waveform),
///     "IntWaveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 2492928}"
/// );
/// println!("{:?}", &int_waveform.interleaved_samples()[30000..30005]);
/// // [783, 2653, 683, 2994, 496]
/// ```
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
            .map(|val| f32_to_i16(*val))
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

impl crate::backend::waveform::Waveform<i16> for IntWaveform {
    /// Constructs an `IntWaveform` from an already-decoded vector of 16-bit integer samples.
    ///
    /// # Arguments
    /// - `frame_rate_hz`: The frame rate of the audio in the `interleaved_samples`
    ///    buffer.
    /// - `num_channels`: The number of audio channels in the `interleaved_samples`
    ///    buffer.
    /// - `interleaved_samples`: An in-memory buffer of *already-decoded* audio.
    ///
    fn new(frame_rate_hz: u32, num_channels: u32, interleaved_samples: Vec<i16>) -> Self {
        let num_frames = interleaved_samples.len() as u64 / num_channels as u64;
        IntWaveform {
            interleaved_samples,
            frame_rate_hz,
            num_channels,
            num_frames,
        }
    }

    /// Returns the frame rate of the `IntWaveform`.
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    /// Returns the total number of channels in the `IntWaveform`.
    fn num_channels(&self) -> u32 {
        self.num_channels
    }

    /// Returns the total number of decoded frames in the `IntWaveform`.
    fn num_frames(&self) -> u64 {
        self.num_frames
    }

    /// Returns the waveform as a slice of channel-interleaved `i16` samples.
    fn interleaved_samples(&self) -> &[i16] {
        &self.interleaved_samples
    }
}
