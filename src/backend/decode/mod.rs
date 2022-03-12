pub mod boxed_decoder;
mod convert_to_mono_decoder_iter;
pub mod decoder;
pub mod decoder_iter;
mod select_channels_decoder_iter;
mod skip_samples_decoder_iter;
pub mod symphonia;
mod take_samples_decoder_iter;

pub use crate::backend::decode::decoder::Decoder;
pub use crate::backend::decode::decoder_iter::DecoderIter;
pub use crate::backend::decode::symphonia::decoder::SymphoniaDecoder;
pub use crate::backend::decode::symphonia::decoder_iter::SymphoniaDecoderIter;

pub use convert_to_mono_decoder_iter::ConvertToMonoDecoderIter;
pub use select_channels_decoder_iter::SelectChannelsDecoderIter;
pub use skip_samples_decoder_iter::SkipSamplesDecoderIter;
pub use take_samples_decoder_iter::TakeSamplesDecoderIter;

#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub mod ffmpeg;
#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub use crate::backend::decode::ffmpeg::decoder::FFmpegDecoder;
#[cfg(all(feature = "enable-filesystem", feature = "enable-ffmpeg"))]
pub use crate::backend::decode::ffmpeg::decoder_iter::FFmpegDecoderIter;

pub use crate::backend::decode::boxed_decoder::from_encoded_bytes;
pub use crate::backend::decode::boxed_decoder::from_encoded_bytes_with_hint;
pub use crate::backend::decode::boxed_decoder::from_encoded_stream;
pub use crate::backend::decode::boxed_decoder::from_encoded_stream_with_hint;

#[cfg(all(feature = "enable-filesystem"))]
pub use crate::backend::decode::boxed_decoder::from_file;
