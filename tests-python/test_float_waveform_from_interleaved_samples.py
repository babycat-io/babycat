"""
Test loading waveforms from 1-D Python lists.
"""
from babycat import FloatWaveform


def test_empty():
    waveform = FloatWaveform.from_interleaved_samples(
        frame_rate_hz=44_100,
        num_channels=3,
        interleaved_samples=[],
    )
    assert waveform.frame_rate_hz == 44_100
    assert waveform.num_channels == 3
    assert waveform.to_interleaved_samples() == []


def test_four_frames_three_channels():
    interleaved_samples = [
        -1.0,
        0.0,
        1.0,
        #
        -1.0,
        0.0,
        1.0,
        #
        -1.0,
        0.0,
        1.0,
        #
        -1.0,
        0.0,
        1.0,
    ]
    waveform = FloatWaveform.from_interleaved_samples(
        frame_rate_hz=44_100,
        num_channels=3,
        interleaved_samples=interleaved_samples,
    )
    assert interleaved_samples == waveform.to_interleaved_samples()
