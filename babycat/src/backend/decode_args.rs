use serde::{Deserialize, Serialize};

pub const DEFAULT_FILE_EXTENSION: &str = "";
pub const DEFAULT_MIME_TYPE: &str = "";
pub const DEFAULT_START_TIME_MILLISECONDS: u64 = 0;
pub const DEFAULT_END_TIME_MILLISECONDS: u64 = 0;
pub const DEFAULT_FRAME_RATE_HZ: u32 = 0;
pub const DEFAULT_NUM_CHANNELS: u32 = 0;
pub const DEFAULT_CONVERT_TO_MONO: bool = false;
pub const DEFAULT_ZERO_PAD_ENDING: bool = false;
pub const DEFAULT_RESAMPLE_MODE: u32 = 0;

pub const RESAMPLE_MODE_LIBSAMPLERATE: u32 = 1;
pub const RESAMPLE_MODE_LANCZOS: u32 = 2;
pub const RESAMPLE_MODE_BABYCAT: u32 = 3;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodeArgs {
    //
    #[serde(default)]
    pub start_time_milliseconds: u64,
    //
    #[serde(default)]
    pub end_time_milliseconds: u64,
    //
    #[serde(default)]
    pub frame_rate_hz: u32,
    //
    #[serde(default)]
    pub num_channels: u32,
    //
    #[serde(default)]
    pub convert_to_mono: bool,
    //
    #[serde(default)]
    pub zero_pad_ending: bool,
    //
    #[serde(default)]
    pub resample_mode: u32,
}

impl Default for DecodeArgs {
    fn default() -> Self {
        DecodeArgs {
            start_time_milliseconds: DEFAULT_START_TIME_MILLISECONDS,
            end_time_milliseconds: DEFAULT_END_TIME_MILLISECONDS,
            frame_rate_hz: DEFAULT_FRAME_RATE_HZ,
            num_channels: DEFAULT_NUM_CHANNELS,
            convert_to_mono: DEFAULT_CONVERT_TO_MONO,
            zero_pad_ending: DEFAULT_ZERO_PAD_ENDING,
            resample_mode: DEFAULT_RESAMPLE_MODE,
        }
    }
}
