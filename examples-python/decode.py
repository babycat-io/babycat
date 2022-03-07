#!/usr/bin/env python3
import babycat


def main():
    try:
        waveform = babycat.Waveform.from_file("audio-for-tests/circus-of-freaks/track.flac")
    except (FileNotFoundError, babycat.exceptions.BabycatError) as exc:
        print("Decoding error:", exc)
        return
    print(
        f"Decoded {waveform.num_frames} frames with "
        f"{waveform.num_channels} channels at "
        f"{waveform.frame_rate_hz} hz"
    )


if __name__ == "__main__":
    main()
