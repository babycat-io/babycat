use serde::{Deserialize, Serialize};

pub const BABYCAT_DEFAULT_NUM_WORKERS: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchArgs {
    pub num_workers: usize,
}

impl Default for BatchArgs {
    fn default() -> Self {
        BatchArgs {
            num_workers: BABYCAT_DEFAULT_NUM_WORKERS as usize,
        }
    }
}
