use serde::{Deserialize, Serialize};

pub const DEFAULT_NUM_WORKERS: u32 = 0;

/// Configures multithreading in Babycat.
///
/// Babycat uses Rayon for multithreading, which
/// [by default](https://github.com/rayon-rs/rayon/blob/master/FAQ.md)
/// will initialize as many threads as *logical* CPU cores on your machine.
/// You can use this `BatchArgs` struct to specify a different
/// number of threads if needed.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchArgs {
    pub num_workers: usize,
}

impl Default for BatchArgs {
    fn default() -> Self {
        BatchArgs {
            num_workers: DEFAULT_NUM_WORKERS as usize,
        }
    }
}
