use serde::{Deserialize, Serialize};

pub const DEFAULT_NUM_WORKERS: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BatchArgs {
    pub num_workers: usize,
}

impl Default for BatchArgs {
    fn default() -> Self {
        BatchArgs {
            num_workers: DEFAULT_NUM_WORKERS as usize
        }
    }
}
