use babycat::{DecodeArgs, FloatWaveform};

fn main() {
    let decode_args = DecodeArgs {
        ..Default::default()
    };
    let waveform =
        match FloatWaveform::from_file("audio-for-tests/circus-of-freaks/track.mp3", decode_args) {
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
