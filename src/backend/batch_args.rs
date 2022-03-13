use serde::{Deserialize, Serialize};

use crate::backend::constants::DEFAULT_NUM_WORKERS;

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
