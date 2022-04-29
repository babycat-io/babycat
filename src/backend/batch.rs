//! Functions that use multithreading to manipulate multiple audio files in parallel.
//!
//! This submodule is only available if the Cargo feature
//! `enable-multithreading` is enabled. Functions that read audio from
//! the filesystem also need the Cargo feature `enable-filesystem`
//! to be enabled. Both of these feature are disabled in Babycat's
//! WebAssembly frontend.
use rayon::prelude::*;

use crate::backend::BatchArgs;
use crate::backend::Waveform;
use crate::backend::WaveformArgs;
use crate::backend::WaveformNamedResult;

/// Decodes a list of audio files in parallel.
///
/// # Arguments
/// - `filenames`: A filename of an encoded audio file on the local filesystem.
/// - `waveform_args`: Instructions on how to demux/decode each audio file.
/// - `batch_args`: Instructions on how to divide the work across multiple threads.
///
/// # Feature flags
/// This function is only available if both of the `enable-filesystem`
/// and `enable-multithreading` features are enabled. These features
/// are enabled by default in Babycat's Rust, Python, and C frontends.
/// These features are disabled in Babycat's WebAssembly frontend.
///
/// # Examples
/// **(Attempt to) decode three files:**
///
/// In this example, we process three filenames and demonstrate how to handle errors.
/// The first two files are successfully processed, and we catch a
/// [`Error::FileNotFound`][crate::Error::FileNotFound] error when processing the third file.
/// ```
/// use babycat::assertions::assert_debug;
/// use babycat::{batch::waveforms_from_files, BatchArgs, Error, WaveformArgs, WaveformNamedResult};
///
/// let filenames = &[
///     "audio-for-tests/andreas-theme/track.flac",
///     "audio-for-tests/blippy-trance/track.wav",
///     "does-not-exist",
/// ];
/// let decode_args = Default::default();
/// let batch_args = Default::default();
/// let batch = waveforms_from_files(
///     filenames,
///     decode_args,
///     batch_args
/// );
///
/// // Check that the FIRST file was SUCCESSFULLY decoded.
/// assert_eq!(&batch[0].name, "audio-for-tests/andreas-theme/track.flac");
/// assert_debug(
///     &batch[0].result,
///     "Ok(Waveform { 9586415 frames,  2 channels,  44100 hz,  3m 37s 379ms })",
/// );
///
/// // Check that the SECOND file was SUCCESSFULLY decoded.
/// assert_eq!(&batch[1].name, "audio-for-tests/blippy-trance/track.wav");
/// assert_debug(
///     &batch[1].result,
///     "Ok(Waveform { 5292911 frames,  2 channels,  44100 hz,  2m 20ms })",
/// );
///
/// // Check that the THIRD file returned a [`Error#FileNotFound`] error.
/// assert_eq!(&batch[2].name, "does-not-exist");
/// assert_debug(
///     &batch[2].result,
///     "Err(FileNotFound(\"does-not-exist\"))",
/// );
/// ```
#[allow(dead_code)] // Silence dead code warning because we do not use this function in the C frontend.
#[allow(clippy::missing_panics_doc)]
pub fn waveforms_from_files(
    filenames: &[&str],
    waveform_args: WaveformArgs,
    batch_args: BatchArgs,
) -> Vec<WaveformNamedResult> {
    let thread_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(batch_args.num_workers)
        .build()
        .unwrap();

    let waveforms: Vec<WaveformNamedResult> = thread_pool.install(|| {
        filenames
            .par_iter()
            .map(|filename| WaveformNamedResult {
                name: (*filename).to_string(),
                result: Waveform::from_file(filename, waveform_args),
            })
            .collect()
    });
    waveforms
}
