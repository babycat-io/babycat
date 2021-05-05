use std::fmt;

use crate::backend::waveform::Waveform;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod test_int_waveform {
    use crate::{FloatWaveform, IntWaveform, Waveform};

    fn generate_int_samples() -> Vec<i16> {
        (-0x8000..0x8000).map(|v| v as i16).collect()
    }

    #[test]
    fn test_bit_transparency_1() {
        // Create an IntWaveform.
        let int_samples: Vec<i16> = generate_int_samples();
        let int_waveform_1 = IntWaveform::new(44100, 2, int_samples.clone());
        assert_eq!(int_waveform_1.num_frames(), 0x8000);
        assert_eq!(int_waveform_1.num_channels(), 2);
        assert_eq!(int_waveform_1.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_1.interleaved_samples(), int_samples);
        // Convert it to a FloatWaveform.
        let float_waveform = FloatWaveform::from(int_waveform_1.clone());
        assert_eq!(float_waveform.num_frames(), 0x8000);
        assert_eq!(float_waveform.num_channels(), 2);
        assert_eq!(float_waveform.frame_rate_hz(), 44100);
        // And convert it back to an IntWaveForm and make sure that nothing has changed.
        let int_waveform_2 = IntWaveform::from(float_waveform);
        assert_eq!(int_waveform_1, int_waveform_2);
        assert_eq!(int_waveform_2.num_frames(), 0x8000);
        assert_eq!(int_waveform_2.num_channels(), 2);
        assert_eq!(int_waveform_2.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_2.interleaved_samples(), int_samples);
    }

    #[test]
    fn test_bit_transparency_silence_1() {
        let int_waveform_1 =
            IntWaveform::from(FloatWaveform::from_milliseconds_of_silence(44100, 2, 30000));
        let int_waveform_2 = IntWaveform::from(FloatWaveform::from(int_waveform_1.clone()));
        assert_eq!(int_waveform_1.num_frames(), 1323000);
        assert_eq!(int_waveform_1.num_channels(), 2);
        assert_eq!(int_waveform_1.frame_rate_hz(), 44100);
        assert_eq!(int_waveform_2.num_frames(), 1323000);
        assert_eq!(int_waveform_2.num_channels(), 2);
        assert_eq!(int_waveform_2.frame_rate_hz(), 44100);
    }

    #[test]
    fn test_serialize_1() {
        // Create an IntWaveform.
        let int_samples: Vec<i16> = generate_int_samples();
        let int_waveform_1 = IntWaveform::new(44100, 2, int_samples.clone());
        // Serialize and deserialize the IntWaveform.
        let int_waveform_json: String = serde_json::to_string(&int_waveform_1).unwrap();
        let int_waveform_deserialized: IntWaveform =
            serde_json::from_str(&int_waveform_json).unwrap();
        // Convert it to a FloatWaveform. Serialize and deserialize it.
        let float_waveform = FloatWaveform::from(int_waveform_1.clone());
        let float_waveform_json: String = serde_json::to_string(&float_waveform).unwrap();
        let float_waveform_deserialized: FloatWaveform =
            serde_json::from_str(&float_waveform_json).unwrap();
        // And convert it back to an IntWaveForm and make sure that nothing has changed.
        // Check that all of our IntWaveforms are the same.
        let int_waveform_2 = IntWaveform::from(float_waveform.clone());
        let int_waveform_2_from_float_waveform_deserialized =
            IntWaveform::from(float_waveform_deserialized);
        assert_eq!(int_waveform_1, int_waveform_2);
        assert_eq!(int_waveform_1, int_waveform_deserialized);
        assert_eq!(
            int_waveform_2,
            int_waveform_2_from_float_waveform_deserialized
        );
    }
}
