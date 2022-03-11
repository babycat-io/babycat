use rayon::prelude::*;

use crate::backend::{
    PoolArgs,
    Waveform,
    WaveformArgs,
    WaveformNamedResult,
};

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
/// use babycat::{Error, WaveformArgs, WaveformNamedResult};
/// use babycat::batch::{PoolArgs, waveforms_from_files};
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
/// fn display_result(wnr: &WaveformNamedResult) -> String {
///     match &wnr.result {
///         Ok(waveform) => format!("\nSuccess: {}:\n{:?}", wnr.name, waveform),
///         Err(err) => format!("\nFailure: {}:\n{}", wnr.name, err),
///     }
/// }
/// assert_eq!(
///     display_result(&batch[0]),
///      "
/// Success: audio-for-tests/andreas-theme/track.flac:
/// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 9586415}",
/// );
/// assert_eq!(
///     display_result(&batch[1]),
///      "
/// Success: audio-for-tests/blippy-trance/track.wav:
/// Waveform { frame_rate_hz: 44100, num_channels: 2, num_frames: 5292911}",
/// );
/// assert_eq!(
///     display_result(&batch[2]),
///      "
/// Failure: does-not-exist:
/// Cannot find the given filename does-not-exist.",
/// );
/// ```
pub struct Pool {
    args: PoolArgs,
    pool: rayon::ThreadPool,
}


impl std::fmt::Debug for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pool {{ args: {:?} }}", self.args)
    }
}

impl Pool {
    #[inline(always)]
    pub fn new(args: PoolArgs) -> Self {
        let pool: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.num_workers)
        .build()
        .unwrap();
        Self {
            pool
        }
    }

    #[inline(always)]
    pub fn new_default() -> Self {
        Self::new(Default::default())
    }

    pub fn waveforms_from_files(
        &self,
        filenames: &[&str],
        waveform_args: WaveformArgs,
    ) -> Vec<WaveformNamedResult> {
        let waveforms: Vec<WaveformNamedResult> = self.pool.install(|| {
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
}