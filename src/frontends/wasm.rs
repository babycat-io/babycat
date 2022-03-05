#![allow(non_snake_case)]

use js_sys::Float32Array;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

fn throw_js_error<E: std::fmt::Display>(err: E) -> JsValue {
    let err_string: String = err.to_string();
    js_sys::Error::new(&err_string).into()
}

/// Docs for Waveform struct.
#[allow(clippy::unused_unit)]
#[wasm_bindgen]
pub struct Waveform {
    inner: crate::backend::Waveform,
}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Waveform {
    /// Creates a silent waveform measured in frames.
    pub fn fromFramesOfSilence(frameRateHz: u32, numChannels: u16, numFrames: usize) -> Self {
        Waveform {
            inner: crate::backend::Waveform::from_frames_of_silence(
                frameRateHz,
                numChannels,
                numFrames as usize,
            ),
        }
    }

    /// Crates a silent waveform measured in milliseconds.
    pub fn fromMillisecondsOfSilence(
        frameRateHz: u32,
        numChannels: u16,
        durationMilliseconds: usize,
    ) -> Self {
        Waveform {
            inner: crate::backend::Waveform::from_milliseconds_of_silence(
                frameRateHz,
                numChannels,
                durationMilliseconds as usize,
            ),
        }
    }

    /// Decodes audio stored in an in-memory byte array.
    pub fn fromEncodedArray(
        encodedArray: Uint8Array,
        WaveformArgs: JsValue,
    ) -> Result<Waveform, JsValue> {
        let parsedWaveformArgs: crate::backend::WaveformArgs = match WaveformArgs.into_serde() {
            Ok(parsed) => parsed,
            Err(err) => return Err(throw_js_error(err)),
        };
        let cursor = std::io::Cursor::new(encodedArray.to_vec());
        match crate::backend::Waveform::from_encoded_stream(cursor, parsedWaveformArgs) {
            Ok(inner) => Ok(Waveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }

    /// Decodes audio using an in-memory byte array, using user-specified encoding hints.
    pub fn fromEncodedArrayWithHint(
        encodedArray: Uint8Array,
        WaveformArgs: JsValue,
        fileExtension: &str,
        mimeType: &str,
    ) -> Result<Waveform, JsValue> {
        let parsedWaveformArgs: crate::backend::WaveformArgs = match WaveformArgs.into_serde() {
            Ok(parsed) => parsed,
            Err(err) => return Err(throw_js_error(err)),
        };
        let cursor = std::io::Cursor::new(encodedArray.to_vec());
        match crate::backend::Waveform::from_encoded_stream_with_hint(
            cursor,
            parsedWaveformArgs,
            fileExtension,
            mimeType,
        ) {
            Ok(inner) => Ok(Waveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }

    /// Encodes the waveform into a WAV-encoded byte array.
    ///
    pub fn toWavBuffer(&self) -> Result<Uint8Array, JsValue> {
        match self.inner.to_wav_buffer() {
            Ok(wav_vec) => {
                let wav_slice: &[u8] = &wav_vec;
                Ok(Uint8Array::from(wav_slice))
            }
            Err(err) => Err(throw_js_error(err)),
        }
    }

    /// Returns channel-interleaved samples.
    pub fn toInterleavedSamples(&self) -> Float32Array {
        Float32Array::from(self.inner.to_interleaved_samples())
    }

    /// Return the frame rate.
    ///
    pub fn frameRateHz(&self) -> u32 {
        self.inner.frame_rate_hz()
    }

    /// Returns the number of channels.
    pub fn numChannels(&self) -> u16 {
        self.inner.num_channels()
    }

    /// Returns the number of frames.
    pub fn numFrames(&self) -> usize {
        self.inner.num_frames()
    }

    /// Resamples the waveform using the default resampler.
    pub fn resample(&self, frameRateHz: u32) -> Result<Waveform, JsValue> {
        match self.inner.resample(frameRateHz) {
            Ok(inner) => Ok(Waveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }

    /// Resamples the audio using a specific resampler.
    pub fn resampleByMode(&self, frameRateHz: u32, resampleMode: u32) -> Result<Waveform, JsValue> {
        match self.inner.resample_by_mode(frameRateHz, resampleMode) {
            Ok(inner) => Ok(Waveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }
}

impl From<crate::backend::Waveform> for Waveform {
    fn from(item: crate::backend::Waveform) -> Self {
        Waveform { inner: item }
    }
}
