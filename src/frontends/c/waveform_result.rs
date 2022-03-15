use crate::backend::{Error, Waveform};
use crate::frontends::c::error::error_to_num;

/// A struct that contains an error value and a pointer to a `babycat_Waveform`.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd)]
pub struct WaveformResult {
    /// The error number.
    ///
    /// This value is either going to be `babycat_NO_ERROR`
    /// or one of the constants with a `babycat_ERROR` prefix.
    error_num: u32,
    /// A pointer to a Waveform.
    result: *mut Waveform,
}

impl From<Waveform> for *mut Waveform {
    fn from(item: Waveform) -> Self {
        Box::into_raw(Box::new(item))
    }
}

impl From<Result<Waveform, Error>> for WaveformResult {
    fn from(item: Result<Waveform, Error>) -> Self {
        match item {
            Ok(result) => {
                let boxed = Box::new(result);
                WaveformResult {
                    error_num: 0,
                    result: Box::into_raw(boxed),
                }
            }
            Err(error) => WaveformResult {
                error_num: error_to_num(error),
                result: std::ptr::null_mut::<Waveform>(),
            },
        }
    }
}
