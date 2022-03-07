pub fn milliseconds_to_frames(frame_rate_hz: u32, duration_milliseconds: usize) -> usize {
    (duration_milliseconds * frame_rate_hz as usize) / 1000
}
