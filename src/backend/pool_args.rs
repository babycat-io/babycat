use serde::{Deserialize, Serialize};


/// The default number of threads to use for multithreaded operations.
/// By default, we will initialize as many threads as *logical*
/// CPU cores on your machine.
pub const DEFAULT_NUM_WORKERS: u32 = 0;

/// Configures multithreading in Babycat.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PoolArgs {
    /// The maximum number of threads to initialize when doing multithreaded work.
    ///
    /// Babycat uses Rayon for multithreading, which
    /// [by default](https://github.com/rayon-rs/rayon/blob/master/FAQ.md)
    /// will initialize as many threads as *logical* CPU cores on your machine.
    pub num_workers: usize,
}

impl Default for PoolArgs {
    fn default() -> Self {
        PoolArgs {
            num_workers: DEFAULT_NUM_WORKERS as usize,
        }
    }
}
