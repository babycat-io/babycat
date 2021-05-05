pub trait Waveform {
    fn frame_rate_hz(&self) -> u32;

    fn num_channels(&self) -> u32;

    fn num_frames(&self) -> u64;
}
