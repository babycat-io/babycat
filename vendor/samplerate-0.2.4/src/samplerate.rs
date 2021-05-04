use crate::converter_type::ConverterType;
use crate::error::{Error, ErrorCode};
use libsamplerate_sys::*;
use std::clone::Clone;

/// A samplerate converter. This is a wrapper around libsamplerate's `SRC_STATE` which also
/// stores the source and target samplerates.
///
/// # Example
///
/// ```
/// use samplerate::{Samplerate, ConverterType};
///
/// // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
/// let freq = std::f32::consts::PI * 880f32 / 44100f32;
/// let mut input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();
///
/// // Instanciate a new converter.
/// let mut converter = Samplerate::new(ConverterType::SincBestQuality, 44100, 48000, 1).unwrap();
///
/// // Resample the input from 44100Hz to 48000Hz.
/// let resampled = converter.process_last(&input).unwrap();
/// assert_eq!(resampled.len(), 48000);
/// ```
pub struct Samplerate {
    ptr: *mut SRC_STATE,
    from_rate: u32,
    to_rate: u32,
}

impl Samplerate {
    /// Create a new samplerate converter with the given rates and channels.
    pub fn new(converter_type: ConverterType, from_rate: u32, to_rate: u32, channels: usize) -> Result<Samplerate, Error> {
        // First, check that the provided ratio is supported by libsamplerate.
        let ratio = to_rate as f64 / from_rate as f64;
        if unsafe { src_is_valid_ratio(ratio) } == 0 {
            return Err(Error::from_code(ErrorCode::BadSrcRatio));
        }
        // Construct the `SRC_STATE` struct and check if that worked.
        let mut error_int = 0i32;
        let ptr: *mut SRC_STATE = unsafe { src_new(converter_type as i32, channels as i32, &mut error_int as *mut i32) };
        match ErrorCode::from_int(error_int) {
            ErrorCode::NoError => {
                Ok(Samplerate {
                    ptr,
                    from_rate,
                    to_rate,
                })
            },
            _ => Err(Error::from_int(error_int)),
        }
    }

    /// Reset the internal converter's state.
    pub fn reset(&mut self) -> Result<(), Error> {
        let error_code = ErrorCode::from_int(unsafe { src_reset(self.ptr) });
        match error_code {
            ErrorCode::NoError => Ok(()),
            _ => Err(Error::from_code(error_code)),
        }
    }

    /// Retrieve the currently used source samplerate.
    pub fn from_rate(&self) -> u32 {
        self.from_rate
    }

    /// Retrieve the currently used target samplerate.
    pub fn to_rate(&self) -> u32 {
        self.to_rate
    }

    /// Change the source samplerate.
    pub fn set_from_rate(&mut self, from_rate: u32) {
        self.from_rate = from_rate;
    }

    /// Change the target samplerate.
    pub fn set_to_rate(&mut self, to_rate: u32) {
        self.to_rate = to_rate;
    }

    /// Calculate the ratio (target samplerate divided by source samplerate).
    pub fn ratio(&self) -> f64 {
        self.to_rate as f64 / self.from_rate as f64
    }

    /// Retrieve the number of channels used.
    pub fn channels(&self) -> Result<usize, Error> {
        let channels = unsafe { src_get_channels(self.ptr) };
        if channels >= 0 {
            Ok(channels as usize)
        } else {
            Err(Error::from_int(channels))
        }
    }

    fn _process(&self, input: &[f32], output_len: usize, end_of_input: bool) -> Result<Vec<f32>, Error> {
        let channels = self.channels()?;
        let mut output = vec![0f32;output_len];
        let mut src = SRC_DATA {
            data_in: input.as_ptr(),
            data_out: output.as_mut_ptr(),
            input_frames: (input.len() as i32 / channels as i32).into(),
            output_frames: (output_len as i32 / channels as i32).into(),
            src_ratio: self.ratio(),
            end_of_input: if end_of_input { 1 } else { 0 },
            input_frames_used: 0,
            output_frames_gen: 0,
            ..Default::default()
        };
        let error_int = unsafe { src_process(self.ptr, &mut src as *mut SRC_DATA) };
        match ErrorCode::from_int(error_int) {
            ErrorCode::NoError => Ok(output[..src.output_frames_gen as usize*channels].into()),
            _ => Err(Error::from_int(error_int)),
        }
    }

    /// Perform a samplerate conversion on a block of data (use `process_last` if it is the last one)
    /// If the number of channels used was not `1` (Mono), the samples are expected to be stored
    /// interleaved.
    pub fn process(&self, input: &[f32]) -> Result<Vec<f32>, Error> {
        let channels = self.channels()?;
        self._process(input, (self.ratio() * input.len() as f64) as usize + channels, false)
    }
    
    /// Perform a samplerate conversion on last block of given input data.
    /// If the number of channels used was not `1` (Mono), the samples are expected to be stored
    /// interleaved.
    pub fn process_last(&self, input: &[f32]) -> Result<Vec<f32>, Error> {
        let channels = self.channels()?;
        let output_len = (self.ratio() * input.len() as f64) as usize + channels;
        match self._process(input, output_len, true) {
            Ok(mut output) => {
                loop {
                    match self._process(&[0f32; 0], output_len, true) {
                        Ok(output_last) => {
                            if output_last.len() < 1 {
                                break;
                            } else {
                                output.extend(output_last);
                            }
                        },
                        Err(err) => return Err(err)
                    }
                }
                Ok(output)
            },
            Err(err) => Err(err)
        }
    }
}

impl Drop for Samplerate {
    fn drop(&mut self) {
        unsafe { src_delete(self.ptr) };
    }
}

impl Clone for Samplerate {
    /// Might panic if the underlying `src_clone` method from libsamplerate returns an error.
    fn clone(&self) -> Samplerate {
        let mut error_int = 0i32;
        let ptr: *mut SRC_STATE = unsafe { src_clone(self.ptr, &mut error_int as *mut i32) };
        let error_code = ErrorCode::from_int(error_int);
        if error_code != ErrorCode::NoError {
            panic!("Error when cloning Samplerate struct: {}", error_code.description());
        }
        Samplerate {
            ptr,
            from_rate: self.from_rate,
            to_rate: self.to_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::usize;

    #[test]
    fn samplerate_new_channels_error() {
        match Samplerate::new(ConverterType::Linear, 44100, 48000, usize::MAX) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, Error::from_code(ErrorCode::BadChannelCount)),
        };
    }

    #[test]
    fn samplerate_new_channels_correct() {
        let converter = Samplerate::new(ConverterType::Linear, 44100, 48000, 4).unwrap();
        assert_eq!(converter.channels().unwrap(), 4);
    }

    #[test]
    fn samplerate_clone() {
        let converter = Samplerate::new(ConverterType::Linear, 44100, 48000, 4).unwrap();
        let cloned = converter.clone();
        assert_eq!(cloned.channels().unwrap(), 4);
    }

    #[test]
    fn samplerate_conversion() {
        // Generate a 880Hz sine wave for 1 second in 44100Hz with one channel.
        let freq = std::f32::consts::PI * 880f32 / 44100f32;
        let input: Vec<f32> = (0..44100).map(|i| (freq * i as f32).sin()).collect();

        // Create a new converter.
        let mut converter = Samplerate::new(ConverterType::SincBestQuality, 44100, 48000, 1).unwrap();

        // Resample the audio in chunks.
        let mut resampled = vec![0f32;0];
        let chunk_size = 4410; // 100ms
        for i in 0..input.len() / chunk_size {
            resampled.extend(if i < (input.len() / chunk_size - 1) {
                converter.process(&input[i * chunk_size .. (i + 1) * chunk_size]).unwrap()
            } else {
                converter.process_last(&input[i * chunk_size .. (i + 1) * chunk_size]).unwrap()
            });
        }
        assert_eq!(resampled.len(), 48000);

        // Resample the audio back.
        converter.reset().unwrap();
        converter.set_to_rate(44100);
        converter.set_from_rate(48000);
        let mut output = vec![0f32;0];
        let chunk_size = 4800; // 100ms
        for i in 0..resampled.len() / chunk_size {
            output.extend(if i < (resampled.len() / chunk_size - 1) {
                converter.process(&resampled[i * chunk_size .. (i + 1) * chunk_size]).unwrap()
            } else {
                converter.process_last(&resampled[i * chunk_size .. (i + 1) * chunk_size]).unwrap()
            });
        }
        assert_eq!(output.len(), 44100);

        // Expect the difference between all input frames and all output frames to be less than
        // an epsilon.
        let error = input.iter().zip(output).fold(0f32, |max, (input, output)| max.max((input - output).abs()));
        assert!(error < 0.002);
    }
}
