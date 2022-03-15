use std::ffi::CStr;
use std::os::raw::c_char;

use crate::backend::{Signal, Waveform, WaveformArgs};

use crate::frontends::c::waveform_result::WaveformResult;

/// Returns a `babycat_WaveformArgs` struct with all default values.
#[no_mangle]
pub extern "C" fn babycat_waveform_args_init_default() -> WaveformArgs {
    WaveformArgs {
        ..Default::default()
    }
}

/// Frees a `babycat_Waveform` struct.
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_free(waveform: *mut Waveform) {
    Box::from_raw(waveform);
}

/// Creates a silent waveform measured in frames.
///
/// @param frame_rate_hz The frame rate of the waveform to create.
/// @param num_channels The number of channels in the waveform to create.
/// @param num_frames The number of frames of audio to generate.
///
#[no_mangle]
pub extern "C" fn babycat_waveform_from_frames_of_silence(
    frame_rate_hz: u32,
    num_channels: u16,
    num_frames: usize,
) -> *mut Waveform {
    Waveform::from_frames_of_silence(frame_rate_hz, num_channels, num_frames).into()
}

/// Create a silent waveform measured in milliseconds.
///
/// @param frame_rate_hz The frame rate of the waveform to create.
/// @param num_channels The number of channels in the waveform to create.
/// @param duration_milliseconds The length of the audio waveform in milliseconds.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub extern "C" fn babycat_waveform_from_milliseconds_of_silence(
    frame_rate_hz: u32,
    num_channels: u16,
    duration_milliseconds: usize,
) -> *mut Waveform {
    Waveform::from_milliseconds_of_silence(frame_rate_hz, num_channels, duration_milliseconds)
        .into()
}

/// Decodes audio in an in-memory byte array, using user-specified encoding hints.
///
/// @param encoded_bytes A byte array containing encoded (e.g. MP3) audio.
/// @param encoded_bytes_len The length of the `encoded_bytes` byte array.
/// @param waveform_args Instructions on how to decode the audio.
/// @param file_extension A hint, in the form of a file extension, to indicate
///        the encoding of the audio in `encoded_bytes`.
/// @param mime_type A hint, in the form of a MIME type, to indicate
///        the encoding of the audio in `encoded_bytes`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_from_encoded_bytes_with_hint(
    encoded_bytes: *mut u8,
    encoded_bytes_len: usize,
    waveform_args: WaveformArgs,
    file_extension: *const c_char,
    mime_type: *const c_char,
) -> WaveformResult {
    let encoded_bytes_vec =
        Vec::<u8>::from_raw_parts(encoded_bytes, encoded_bytes_len, encoded_bytes_len);
    let file_extension_str = CStr::from_ptr(file_extension).to_str().unwrap();
    let mime_type_str = CStr::from_ptr(mime_type).to_str().unwrap();
    Waveform::from_encoded_bytes_with_hint(
        &encoded_bytes_vec,
        waveform_args,
        file_extension_str,
        mime_type_str,
    )
    .into()
}

/// Decodes audio stored in an in-memory byte array.
///
/// @param encoded_bytes A byte array containing encoded (e.g. MP3) audio.
/// @param encoded_bytes_len The length of the `encoded_bytes` byte array.
/// @param waveform_args Instructions on how to decode the audio.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_from_encoded_bytes(
    encoded_bytes: *mut u8,
    encoded_bytes_len: usize,
    waveform_args: WaveformArgs,
) -> WaveformResult {
    let encoded_bytes_vec =
        Vec::<u8>::from_raw_parts(encoded_bytes, encoded_bytes_len, encoded_bytes_len);
    Waveform::from_encoded_bytes(&encoded_bytes_vec, waveform_args).into()
}

/// Decodes audio stored in a local file.
///
/// @param filename A filename of an encoded audio file on the local filesystem.
/// @param waveform_args Instructions on how to decode the audio.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_from_file(
    filename: *const c_char,
    waveform_args: WaveformArgs,
) -> WaveformResult {
    let filename_rust = CStr::from_ptr(filename).to_str().unwrap();
    Waveform::from_file(filename_rust, waveform_args).into()
}

/// Returns the frame rate of an existing `babycat_Waveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_get_frame_rate_hz(waveform: *mut Waveform) -> u32 {
    (*(waveform)).frame_rate_hz()
}

/// Returns the number of channels of an existing `babycat_Waveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_get_num_channels(waveform: *mut Waveform) -> u16 {
    (*(waveform)).num_channels()
}

/// Returns the number of frames in an existing `babycat_Waveform`.
///
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_get_num_frames(waveform: *mut Waveform) -> usize {
    (*(waveform)).num_frames()
}

/// Returns the number of samples in an existing `babycat_Waveform`.
///
/// @param waveform A pointer to the `babycat_Waveform`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_get_num_samples(waveform: *mut Waveform) -> usize {
    let w = &*(waveform);
    w.num_frames() * w.num_channels() as usize
}

/// Returns a pointer to an in-memory array of interleaved audio samples.
///
/// @param waveform A pointer to the `babycat_Waveform`.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_to_interleaved_samples(
    waveform: *mut Waveform,
) -> *const f32 {
    waveform.as_ref().unwrap().to_interleaved_samples().as_ptr()
}

/// Resample a `babycat_Waveform` with the default resampler.
///
/// @param waveform A pointer to the `babycat_Waveform` to resample.
/// @param frame_rate_hz The destination frame rate to resample to.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_resample(
    waveform: *mut Waveform,
    frame_rate_hz: u32,
) -> WaveformResult {
    (*(waveform)).resample(frame_rate_hz).into()
}

/// Resamples a `babycat_Waveform` using a specific resampler.
///
/// @param waveform A pointer to the `babycat_Waveform` to resample.
/// @param frame_rate_hz The destination frame rate to resample to.
/// @param resample_mode The Babycat resampling backend to pick.
///
#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn babycat_waveform_resample_by_mode(
    waveform: *mut Waveform,
    frame_rate_hz: u32,
    resample_mode: u32,
) -> WaveformResult {
    (*(waveform))
        .resample_by_mode(frame_rate_hz, resample_mode)
        .into()
}
