use babycat::{Waveform, WaveformArgs};

fn main() {
    let waveform_args = WaveformArgs {
        ..Default::default()
    };
    let waveform =
        match Waveform::from_file("audio-for-tests/circus-of-freaks/track.flac", waveform_args) {
            Ok(w) => w,
            Err(err) => {
                println!("Decoding error: {}", err);
                return;
            }
        };
    println!(
        "Decoded {} frames with {} channels at {} hz",
        waveform.num_frames(),
        waveform.num_channels(),
        waveform.frame_rate_hz(),
    );
}
