pub trait Signal {
    fn frame_rate_hz(&self) -> u32;

    fn num_channels(&self) -> u16;

    fn num_frames_estimate(&self) -> Option<usize>;
}
