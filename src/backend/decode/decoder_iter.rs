use crate::backend::signal::Signal;

/// A sample iterator created by an audio decoder.
pub trait DecoderIter: Signal + Iterator<Item = f32> {}
