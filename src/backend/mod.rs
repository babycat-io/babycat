#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
pub mod batch;

#[doc(hidden)]
pub mod resample;

pub mod assertions;
pub mod build_info;
pub mod constants;
pub mod decoder;
pub mod display;
pub mod source;
pub mod units;

mod batch_args;
mod errors;
mod sample;
mod signal;
mod waveform;
mod waveform_args;
mod waveform_named_result;
mod waveform_result;

pub use batch_args::BatchArgs;
pub use errors::Error;
pub use sample::Sample;
pub use signal::Signal;
pub use source::Source;
pub use source::WaveformSource;
pub use waveform::Waveform;
pub use waveform_args::WaveformArgs;
pub use waveform_named_result::WaveformNamedResult;
pub use waveform_result::WaveformResult;
