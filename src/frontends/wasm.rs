#![allow(non_snake_case)]

use babycat::Waveform;
use js_sys::Float32Array;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

fn throw_js_error<E: std::fmt::Display>(err: E) -> JsValue {
    let err_string: String = err.to_string();
    js_sys::Error::new(&err_string).into()
}

#[wasm_bindgen]
pub struct FloatWaveform {
    inner: babycat::FloatWaveform,
}

#[wasm_bindgen]
impl FloatWaveform {
    pub fn fromFramesOfSilence(frameRateHz: u32, numChannels: u32, numFrames: u32) -> Self {
        FloatWaveform {
            inner: babycat::FloatWaveform::from_frames_of_silence(
                frameRateHz,
                numChannels,
                numFrames as u64,
            ),
        }
    }

    pub fn fromMillisecondsOfSilence(
        frameRateHz: u32,
        numChannels: u32,
        durationMilliseconds: u32,
    ) -> Self {
        FloatWaveform {
            inner: babycat::FloatWaveform::from_milliseconds_of_silence(
                frameRateHz,
                numChannels,
                durationMilliseconds as u64,
            ),
        }
    }

    pub fn fromEncodedArray(
        encodedArray: Uint8Array,
        decodeArgs: JsValue,
    ) -> Result<FloatWaveform, JsValue> {
        let parsedDecodeArgs: babycat::DecodeArgs = match decodeArgs.into_serde() {
            Ok(parsed) => parsed,
            Err(err) => return Err(throw_js_error(err)),
        };
        let cursor = std::io::Cursor::new(encodedArray.to_vec());
        match babycat::FloatWaveform::from_encoded_stream(cursor, parsedDecodeArgs) {
            Ok(inner) => Ok(FloatWaveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }

    pub fn fromEncodedArrayWithHint(
        encodedArray: Uint8Array,
        decodeArgs: JsValue,
        fileExtension: &str,
        mimeType: &str,
    ) -> Result<FloatWaveform, JsValue> {
        let parsedDecodeArgs: babycat::DecodeArgs = match decodeArgs.into_serde() {
            Ok(parsed) => parsed,
            Err(err) => return Err(throw_js_error(err)),
        };
        let cursor = std::io::Cursor::new(encodedArray.to_vec());
        match babycat::FloatWaveform::from_encoded_stream_with_hint(
            cursor,
            parsedDecodeArgs,
            fileExtension,
            mimeType,
        ) {
            Ok(inner) => Ok(FloatWaveform { inner }),
            Err(err) => Err(throw_js_error(err)),
        }
    }

    pub fn toWavBuffer(&self) -> Result<Uint8Array, JsValue> {
        match self.inner.to_wav_buffer() {
            Ok(wav_vec) => {
                let wav_slice: &[u8] = &wav_vec;
                Ok(Uint8Array::from(wav_slice))
            }
            Err(err) => Err(throw_js_error(err)),
        }
    }

    pub fn interleavedSamples(&self) -> Float32Array {
        Float32Array::from(self.inner.interleaved_samples())
    }

    pub fn frameRateHz(&self) -> u32 {
        self.inner.frame_rate_hz()
    }

    pub fn numChannels(&self) -> u32 {
        self.inner.num_channels()
    }

    pub fn numFrames(&self) -> u32 {
        self.inner.num_frames() as u32
    }
}

impl From<babycat::FloatWaveform> for FloatWaveform {
    fn from(item: babycat::FloatWaveform) -> Self {
        FloatWaveform { inner: item }
    }
}
