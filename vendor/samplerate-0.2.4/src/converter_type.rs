use libsamplerate_sys::*;
use std::ffi::CStr;
use crate::error::{Error, ErrorCode};

/// A converter type used to distinguish the interpolation function used by libsamplerate.
/// Has a great impact on quality and performance.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConverterType {
    SincBestQuality = SRC_SINC_BEST_QUALITY as isize,
    SincMediumQuality = SRC_SINC_MEDIUM_QUALITY as isize,
    SincFastest = SRC_SINC_FASTEST as isize,
    ZeroOrderHold = SRC_ZERO_ORDER_HOLD as isize,
    Linear = SRC_LINEAR as isize,
}

impl ConverterType {
    /// Create a new `ConverterType` enum from the corresponding integer.
    pub fn from_int(value: isize) -> Result<ConverterType, Error> {
        match value {
            0 => Ok(ConverterType::SincBestQuality),
            1 => Ok(ConverterType::SincMediumQuality),
            2 => Ok(ConverterType::SincFastest),
            3 => Ok(ConverterType::ZeroOrderHold),
            4 => Ok(ConverterType::Linear),
            _ => Err(Error::from_code(ErrorCode::BadConverter)),
        }
    }

    /// Return a human-readable name for this type of converter.
    pub fn name(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(src_get_name(*self as i32))
        }.to_str().unwrap()
    }

    /// Return the human-readable description for this type of converter.
    pub fn description(&self) -> &'static str {
        unsafe {
            CStr::from_ptr(src_get_description(*self as i32))
        }.to_str().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_converter_type_from_int() {
        assert_eq!(ConverterType::from_int(0), Ok(ConverterType::SincBestQuality));
        assert_eq!(ConverterType::from_int(1), Ok(ConverterType::SincMediumQuality));
        assert_eq!(ConverterType::from_int(2), Ok(ConverterType::SincFastest));
        assert_eq!(ConverterType::from_int(3), Ok(ConverterType::ZeroOrderHold));
        assert_eq!(ConverterType::from_int(4), Ok(ConverterType::Linear));
        assert_eq!(ConverterType::from_int(8), Err(Error::from_code(ErrorCode::BadConverter)));
    }

    #[test]
    fn name() {
        assert_eq!(ConverterType::SincBestQuality.name(), "Best Sinc Interpolator");
        assert_eq!(ConverterType::SincMediumQuality.name(), "Medium Sinc Interpolator");
        assert_eq!(ConverterType::SincFastest.name(), "Fastest Sinc Interpolator");
        assert_eq!(ConverterType::ZeroOrderHold.name(), "ZOH Interpolator");
        assert_eq!(ConverterType::Linear.name(), "Linear Interpolator");
    }

    #[test]
    fn description() {
        assert_eq!(ConverterType::SincBestQuality.description(), "Band limited sinc interpolation, best quality, 144dB SNR, 96% BW.");
        assert_eq!(ConverterType::SincMediumQuality.description(), "Band limited sinc interpolation, medium quality, 121dB SNR, 90% BW.");
        assert_eq!(ConverterType::SincFastest.description(), "Band limited sinc interpolation, fastest, 97dB SNR, 80% BW.");
        assert_eq!(ConverterType::ZeroOrderHold.description(), "Zero order hold interpolator, very fast, poor quality.");
        assert_eq!(ConverterType::Linear.description(), "Linear interpolator, very fast, poor quality.");
    }
}
