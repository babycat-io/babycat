/// Methods common to all types of waveforms.
pub trait Waveform<T> {
    fn new(frame_rate_hz: u32, num_channels: u32, interleaved_samples: Vec<T>) -> Self;

    /// The frame rate (or sample rate) of the audio in memory.
    ///
    /// This returns how many audio frames represent one second of audio.
    fn frame_rate_hz(&self) -> u32;

    /// The number of audio channels.
    fn num_channels(&self) -> u32;

    /// The number of frames in the audio.
    ///
    /// Babycat defines a frame as a collection of time-coincidant
    /// samples--one sample for every channel.
    /// Therefore, the total number of samples
    /// is `num_frames * num_channels`.
    fn num_frames(&self) -> u64;

    /// Return the waveform as a slice of interleaved samples.
    fn to_interleaved_samples(&self) -> &[T];
}
