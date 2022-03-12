#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
extern crate ffmpeg_next;

#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub mod ffmpeg;

#[cfg(all(feature = "enable-multithreading", feature = "enable-filesystem"))]
pub mod batch;

#[doc(hidden)]
pub mod resample;

mod batch_args;
pub mod build_info;
mod common;
pub mod constants;
pub mod decoder;
mod decoder_iter;
mod errors;
mod sample;
mod signal;
pub mod symphonia;
mod waveform;
mod waveform_args;
mod waveform_named_result;
mod waveform_result;

pub use batch_args::BatchArgs;
pub use decoder::Decoder;
pub use decoder_iter::DecoderIter;
pub use errors::Error;
pub use sample::Sample;
pub use signal::Signal;
pub use waveform::Waveform;
pub use waveform_args::WaveformArgs;
pub use waveform_named_result::WaveformNamedResult;
pub use waveform_result::WaveformResult;
