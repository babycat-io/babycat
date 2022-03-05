extern crate ffmpeg_next as ffmpeg;

#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
pub mod batch;
mod common;
pub mod decode;
mod decode_args;
mod errors;
pub mod resample;
mod waveform;
mod waveform_args;
mod waveform_named_result;
mod waveform_result;

pub use decode_args::DecodeArgs;
pub use errors::*;
pub use waveform::*;
pub use waveform_args::*;
pub use waveform_named_result::WaveformNamedResult;
pub use waveform_result::WaveformResult;
