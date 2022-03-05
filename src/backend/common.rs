pub fn milliseconds_to_frames(frame_rate_hz: u32, duration_milliseconds: usize) -> usize {
    (duration_milliseconds * frame_rate_hz as usize) / 1000
}

pub fn get_est_num_frames(
    original_est_num_frames: usize,
    start_frame_idx: usize,
    end_frame_idx: usize,
) -> usize {
    let est_end_frame_idx: usize = if end_frame_idx == 0 {
        original_est_num_frames
    } else {
        std::cmp::min(original_est_num_frames, end_frame_idx)
    };
    if start_frame_idx >= est_end_frame_idx {
        0
    } else {
        est_end_frame_idx - start_frame_idx
    }
}
