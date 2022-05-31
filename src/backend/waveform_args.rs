use serde::{Deserialize, Serialize};

use crate::backend::constants::{
    DEFAULT_CONVERT_TO_MONO, DEFAULT_DECODING_BACKEND, DEFAULT_END_TIME_MILLISECONDS,
    DEFAULT_FRAME_RATE_HZ, DEFAULT_NUM_CHANNELS, DEFAULT_REPEAT_PAD_ENDING, DEFAULT_RESAMPLE_MODE,
    DEFAULT_START_TIME_MILLISECONDS, DEFAULT_ZERO_PAD_ENDING,
};
/// Specifies what transformations to apply to the audio during the decoding
/// process.
///
/// The default value for this struct will tell Babycat to decode audio
/// as-is and not change anything.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaveformArgs {
    /// We discard any audio before this millisecond
    /// offset. By default, this does nothing and the
    /// audio is decoded from the beginning.
    /// Negative offsets are invalid.
    #[serde(default)]
    pub start_time_milliseconds: usize,
    /// We discard any audio after this millisecond offset. By default,
    /// this does nothing and the audio is decoded all the way
    /// to the end. If
    /// [`start_time_milliseconds`](#structfield.start_time_milliseconds)
    /// is specified, then
    /// [`end_time_milliseconds`](#structfield.end_time_milliseconds)
    /// must be greater.
    #[serde(default)]
    pub end_time_milliseconds: usize,
    /// A destination frame rate to resample
    /// the audio to. Do not specify this parameter if you wish
    /// Babycat to preserve the audio's original frame rate.
    /// This does nothing if [`frame_rate_hz`](#structfield.frame_rate_hz)
    /// is equal to the audio's original frame rate.
    #[serde(default)]
    pub frame_rate_hz: u32,
    /// Set this to a positive integer `n`
    /// to select the *first* `n` channels stored in the
    /// audio file. By default, Babycat will return all of the channels
    /// in the original audio. This will raise an exception
    /// if you specify a [`num_channels`](#structfield.num_channels)
    /// greater than the actual number of channels in the audio.
    #[serde(default)]
    pub num_channels: u16,
    /// Set to `true` to average all channels
    /// into a single monophonic (mono) channel. If
    /// `num_channels = n` is also specified, then only the
    /// first `n` channels will be averaged. Note that
    /// [`convert_to_mono`](#structfield.convert_to_mono)
    /// cannot be set to `true` while
    /// also setting `num_channels = 1`.
    #[serde(default)]
    pub convert_to_mono: bool,
    /// If you set this to `true`,
    /// Babycat will zero-pad the ending of the decoded waveform
    /// to ensure that the output waveform's duration is exactly
    /// `end_time_milliseconds - start_time_milliseconds`.
    /// By default, `zero_pad_ending = false`, in which case
    /// the output waveform will be shorter than
    /// `end_time_milliseconds - start_time_milliseconds`
    /// if the input audio is shorter than `end_time_milliseconds`.
    /// Note that setting `zero_pad_ending = true` is
    /// mutually exclusive with setting `repeat_pad_ending = true`.
    #[serde(default)]
    pub zero_pad_ending: bool,
    /// If you set this to `true`,
    /// Babycat will repeat the audio waveform to ensure that
    /// the output waveform's duration is exactly
    /// `end_time_milliseconds - start_time_milliseconds`.
    /// By default, `repeat_pad_ending = false`, in which
    /// case the output waveform will be shorter than
    /// `end_time_milliseconds - start_time_milliseconds`
    /// if the input audio is shorter than `end_time_milliseconds`.
    /// Note that setting `repeat_pad_ending = true` is
    /// mutually exclusive with setting `zero_pad_ending = true`.
    #[serde(default)]
    pub repeat_pad_ending: bool,
    /// Sets which resampling method is used if you have set
    /// [`frame_rate_hz`](#structfield.frame_rate_hz).
    /// This usually defaults to the highest-accuracy resampler compiled
    /// into Babycat. The available choices are:
    /// * [`RESAMPLE_MODE_LIBSAMPLERATE`](crate::constants::RESAMPLE_MODE_LIBSAMPLERATE)
    /// * [`RESAMPLE_MODE_BABYCAT_LANCZOS`](crate::constants::RESAMPLE_MODE_BABYCAT_LANCZOS)
    /// * [`RESAMPLE_MODE_BABYCAT_SINC`](crate::constants::RESAMPLE_MODE_BABYCAT_SINC)
    ///
    #[serde(default)]
    pub resample_mode: u32,
    #[serde(default)]
    /// Sets which audio decoding backend to use.
    /// Currently the only available decoding backend is
    /// [`DECODING_BACKEND_SYMPHONIA`](crate::constants::DECODING_BACKEND_SYMPHONIA), which
    /// corresponds to the [`SymphoniaDecoder`](crate::decoder::SymphoniaDecoder), which
    /// is a wrapper for the [`symphonia`](https://github.com/pdeljanov/Symphonia/) library.
    pub decoding_backend: u32,
}

impl Default for WaveformArgs {
    fn default() -> Self {
        WaveformArgs {
            start_time_milliseconds: DEFAULT_START_TIME_MILLISECONDS,
            end_time_milliseconds: DEFAULT_END_TIME_MILLISECONDS,
            frame_rate_hz: DEFAULT_FRAME_RATE_HZ,
            num_channels: DEFAULT_NUM_CHANNELS,
            convert_to_mono: DEFAULT_CONVERT_TO_MONO,
            zero_pad_ending: DEFAULT_ZERO_PAD_ENDING,
            repeat_pad_ending: DEFAULT_REPEAT_PAD_ENDING,
            resample_mode: DEFAULT_RESAMPLE_MODE,
            decoding_backend: DEFAULT_DECODING_BACKEND,
        }
    }
}

impl WaveformArgs {
    /// Set the [`start_time_milliseconds`](#structfield.start_time_milliseconds) field.
    #[must_use]
    pub fn set_start_time_milliseconds(&mut self, start_time_milliseconds: usize) -> Self {
        self.start_time_milliseconds = start_time_milliseconds;
        *self
    }

    /// Set the [`end_time_milliseconds`](#structfield.end_time_milliseconds) field.
    #[must_use]
    pub fn set_end_time_milliseconds(&mut self, end_time_milliseconds: usize) -> Self {
        self.end_time_milliseconds = end_time_milliseconds;
        *self
    }

    /// Set the [`frame_rate_hz`](#structfield.frame_rate_hz) field.
    #[must_use]
    pub fn set_frame_rate_hz(&mut self, frame_rate_hz: u32) -> Self {
        self.frame_rate_hz = frame_rate_hz;
        *self
    }

    /// Set the [`num_channels`](#structfield.num_channels) field.
    #[must_use]
    pub fn set_num_channels(&mut self, num_channels: u16) -> Self {
        self.num_channels = num_channels;
        *self
    }

    /// Set the [`convert_to_mono`](#structfield.convert_to_mono) field.
    #[must_use]
    pub fn set_convert_to_mono(&mut self, convert_to_mono: bool) -> Self {
        self.convert_to_mono = convert_to_mono;
        *self
    }

    /// Set the [`zero_pad_ending`](#structfield.zero_pad_ending) field.
    #[must_use]
    pub fn set_zero_pad_ending(&mut self, zero_pad_ending: bool) -> Self {
        self.zero_pad_ending = zero_pad_ending;
        *self
    }

    /// Set the [`resample_mode`](#structfield.resample_mode) field.
    #[must_use]
    pub fn set_resample_mode(&mut self, resample_mode: u32) -> Self {
        self.resample_mode = resample_mode;
        *self
    }

    /// Set the [`decoding_backend`](#structfield.decoding_backend) field.
    #[must_use]
    pub fn set_decoding_backend(&mut self, decoding_backend: u32) -> Self {
        self.decoding_backend = decoding_backend;
        *self
    }
}
