pub fn milliseconds_to_frames(frame_rate_hz: u32, duration_milliseconds: u64) -> u64 {
    (duration_milliseconds * frame_rate_hz as u64) / 1000
}
