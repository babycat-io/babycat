//! Functions that use multithreading to manipulate multiple audio files in parallel.
//!
//! This submodule is only available if the Cargo feature
//! `enable-multithreading` is enabled. Functions that read audio from
//! the filesystem also need the Cargo feature `enable-filesystem`
//! to be enabled. Both of these feature are disabled in Babycat's
//! WebAssembly frontend.
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::Waveform;
use crate::backend::WaveformArgs;
use crate::backend::WaveformNamedResult;

/// The default number of threads to use for multithreaded operations.
/// By default, we will initialize as many threads as *logical*
/// CPU cores on your machine.
pub const DEFAULT_NUM_WORKERS: u32 = 0;

/// Configures multithreading in Babycat.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchArgs {
    /// The maximum number of threads to initialize when doing multithreaded work.
    ///
    /// Babycat uses Rayon for multithreading, which
    /// [by default](https://github.com/rayon-rs/rayon/blob/master/FAQ.md)
    /// will initialize as many threads as *logical* CPU cores on your machine.
    pub num_workers: usize,
}

impl Default for BatchArgs {
    fn default() -> Self {
        BatchArgs {
            num_workers: DEFAULT_NUM_WORKERS as usize,
        }
    }
}

/// Decodes multiple audio waveforms into separate [`Waveform`](crate::Waveform)
/// structs.
///
/// This struct and its attached methods are used to decode multiple audio
/// files in parallel using multithreading.
///
/// # Feature flags
/// This function is only available if both of the `enable-filesystem`
/// and `enable-multithreading` features are enabled. These features
/// are enabled by default in Babycat's Rust, Python, and C frontends.
/// These features are disabled in Babycat's WebAssembly frontend.
#[derive(Clone, Debug)]
pub struct BatchProcessor {
    _batch_args: BatchArgs,
    _thread_pool: rayon::ThreadPool,
}

impl BatchProcessor {
    /// Creates a new `BatchProcessor` struct, initializing an internal pool
    /// of worker threads.
    pub fn new(batch_args: BatchArgs) -> Self {
        let thread_pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
            .num_threads(batch_args.num_workers)
            .build()
            .unwrap();

        Self {
            _batch_args: batch_args,
            _thread_pool: thread_pool,
        }
    }

    /// Lists the number of workers threads in the thread pool.
    pub fn current_num_workers(&self) -> usize {
        self._thread_pool.current_num_threads()
    }

    pub fn run_in_pool<OP, R>(&self, op: OP) -> R
    where
        OP: FnOnce() -> R + Send,
        R: Send,
    {
        self._thread_pool.install(op)
    }

    /// Decodes a list of audio files in parallel into separate [`Waveform`](crate::Waveform)
    /// objects.
    ///
    /// # Arguments
    /// - `filenames` A list of filenames of where audio files can be found
    ///  on the local filesystem.
    /// - `waveform_args`: A [`WaveformArgs`](crate::WaveformArgs) instance that
    ///  configures how each individual audio file gets decoded into a
    ///  [`Waveform`](crate::Waveform)
    ///
    /// # Examples
    /// **(Attempt to) decode three files:**
    ///
    /// In this example, we process three filenames and demonstrate how to handle errors.
    /// The first two files are successfully processed, and we catch a
    /// [`Error::FileNotFound`][crate::Error::FileNotFound] error when processing the third file.
    /// ```
    /// use babycat::{Error, WaveformArgs, WaveformNamedResult};
    /// use babycat::batch::{BatchArgs, BatchProcessor};
    ///
    /// let filenames = &[
    ///     "audio-for-tests/andreas-theme/track.mp3",
    ///     "audio-for-tests/blippy-trance/track.mp3",
    ///     "does-not-exist",
    /// ];
    /// let decode_args = Default::default();
    /// let batch_args = Default::default();
    /// let batch_processor = BatchProcessor::new(batch_args);
    /// let batch = batch_processor.waveforms_from_files(
    ///     filenames,
    ///     decode_args
    /// );
    ///
    /// fn display_result(wnr: &WaveformNamedResult) -> String {
    ///     match &wnr.result {
    ///         Ok(waveform) => format!("\nSuccess: {}:\n{:?}", wnr.name, waveform),
    ///         Err(err) => format!("\nFailure: {}:\n{}", wnr.name, err),
    ///     }
    /// }
    /// assert_eq!(
    ///     display_result(&batch[0]),
    ///      "
    /// Success: audio-for-tests/andreas-theme/track.mp3:
    /// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586944}",
    /// );
    /// assert_eq!(
    ///     display_result(&batch[1]),
    ///      "
    /// Success: audio-for-tests/blippy-trance/track.mp3:
    /// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 5293440}",
    /// );
    /// assert_eq!(
    ///     display_result(&batch[2]),
    ///      "
    /// Failure: does-not-exist:
    /// Cannot find the given filename does-not-exist.",
    /// );
    /// ```
    pub fn from_files(
        &self,
        filenames: &[&str],
        waveform_args: WaveformArgs,
    ) -> Vec<WaveformNamedResult> {
        self.run_in_pool(|| {
            filenames
                .par_iter()
                .map(|filename| WaveformNamedResult {
                    name: (*filename).to_string(),
                    result: Waveform::from_file(filename, waveform_args),
                })
                .collect()
        })
    }
}
