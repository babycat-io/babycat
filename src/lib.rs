//! Babycat is a library that makes it easy to decode and manipulate
//! many audio files at once.
//!
//! Babycat is built on top of a lot of other Rust and C libraries, such as:
//! * [Symphonia](https://github.com/pdeljanov/Symphonia) for audio decoding
//! * [libsamplerate](http://www.mega-nerd.com/SRC/index.html) for audio resampling
//! * [Hound](https://github.com/ruuda/hound) for encoding to WAV
//!
//! Babycat provides a consistent audio API for performing many tasks.
//! You are viewing Babycat's Rust documentation, but Babycat also has bindings
//! for C, Python, and WebAssembly.
//!
//! # Terminology
//!Babycat uses the [same audio terminology as the Apple Core Audio API](https://developer.apple.com/documentation/coreaudiotypes/audiostreambasicdescription?language=objc):
//!
//* An audio **stream** is a continuous series of data that represents a sound, such as a song.
//! * A **channel** is a discrete track of monophonic audio. A monophonic stream has one channel. A stereo stream has two channels. An audio stream that contains 5.1 surround sound will have five normal channels and one Low Frequency Enhancement (LFE) channel.
//! * A **sample** is single numerical value in a single audio channel in an audio stream.
//! * A **frame** is a collection of samples from the same point in time--one sample for each channel.
//! * The **frame rate** (or **sample rate**) for a stream is the number of frames per second (hertz) of uncompressed audio.
//!
//! Babycat stores audio as a single array of **interleaved** samples from channels. A waveform with two channels (e.g. left and right) are stored as:
//!
//! # Resampling backends
//! Babycat has the ability to resample audio from one frame rate to another.
//! There are several resampling backends available:
//! Current valid values include:
//!
//! * [`babycat::RESAMPLE_MODE_LIBSAMPLERATE`](crate::RESAMPLE_MODE_LIBSAMPLERATE):
//!   This uses [libsamplerate](http://www.mega-nerd.com/SRC/) at the
//!   `SRC_SINC_BEST_QUALITY` setting. This is the highest-quality resampler
//!   currently offered by Babycat, although it is slightly slower than the other
//!   resamplers. This resampler is only available when Babycat is compiled with
//!  the Cargo feature `enable-libsamplerate` enabled. This feature is enabled
//!   by default in Babycat's C, Python, and Rust frontends. The libsamplerate
//!   resampler is currently unavailable in Babycat's WebAssembly frontend
//!   because libsamplerate's dependency on libc makes it hard to compile
//!   it to the `wasm32-unknown-unknown` target.
//!
//! * [`babycat::RESAMPLE_MODE_BABYCAT_LANCZOS`](crate::RESAMPLE_MODE_BABYCAT_LANCZOS):
//!   A Lanczos resampler to use when compiling to targets like
//!   `wasm32-unknown-unknown` where libsamplerate cannot be compiled to.
//!   This is a simple impmenentation of a
//!   [Lanczos resampler](https://en.wikipedia.org/wiki/Lanczos_resampling).
//!   This is the fastest (and lowest-quality) resampler available in Babycat.
//!
//! * [`babycat::RESAMPLE_MODE_BABYCAT_SINC`](crate::RESAMPLE_MODE_BABYCAT_SINC):
//!   This is an implementation of a sinc resampler
//!   [as described by Stanford professor Julius O. Smith](https://ccrma.stanford.edu/~jos/resample/).
//!   The speed and quality of this resampler is in between the above two.
//!
//! # Examples
//! **Decode multiple audio files in parallel.**
//! ```
//! use babycat::{DecodeArgs, FloatWaveform, Waveform};
//!
//! // These are test files in the Babycat Git repository.
//! let filenames = &[
//!    "audio-for-tests/andreas-theme/track.mp3",
//!    "audio-for-tests/blippy-trance/track.mp3",
//!    "audio-for-tests/voxel-revolution/track.mp3",
//! ];
//!
//! // Perform the following transformations on EACH track.
//! let decode_args = DecodeArgs {
//!     // Upsample the audio to 48khz.
//!     frame_rate_hz: 48000,
//!     // Average all audio channels into a single monophonic channel.
//!     convert_to_mono: true,
//!     // Only select the first 60 seconds of audio.
//!     end_time_milliseconds: 60000,
//!     // If a track is shorter than 60 seconds, pad it with silence.
//!     zero_pad_ending: true,
//!     ..Default::default()
//! };
//! let batch_args = Default::default();
//!
//! // Read and decode the tracks in parallel.
//! let batch = babycat::FloatWaveform::from_many_files(
//!    filenames,
//!    decode_args,
//!    batch_args,
//! );
//!
//! // Iterate over the results.
//! for named_result in batch {
//!     match &named_result.result {
//!         Ok(waveform) => {
//!             // Do further processing.
//!             waveform.interleaved_samples();
//!         }
//!         Err(err) => {
//!             // Handle decoding errors.
//!         }
//!     }
//! }
//! ```

// Leak a str or String value.
// This is useful for passing string values to an error enum.
macro_rules! leak_str {
    ($a:expr) => {
        Box::leak($a.to_owned().into_boxed_str())
    };
}

mod backend;

#[cfg(feature = "frontend-rust")]
pub use crate::backend::*;

#[cfg(any(
    feature = "frontend-c",
    feature = "frontend-python",
    feature = "frontend-wasm"
))]
pub mod frontends;
