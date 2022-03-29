"""
Test loading waveforms from 1-D Python lists.
"""
import numpy as np

from babycat import Waveform


def test_empty():
    waveform = Waveform.from_interleaved_samples(
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
    waveform = Waveform.from_interleaved_samples(
        frame_rate_hz=44_100,
        num_channels=3,
        interleaved_samples=interleaved_samples,
    )
    assert interleaved_samples == waveform.to_interleaved_samples()


def test_five_frames_three_channels_1():
    interleaved_samples = [
        -1.0,
        -0.9,
        -0.8,
        -0.7,
        -0.6,
        -0.5,
        -0.4,
        -0.3,
        -0.2,
        -0.1,
        0.0,
        0.1,
        0.2,
        0.3,
        0.4,
    ]
    waveform = Waveform.from_interleaved_samples(
        frame_rate_hz=44_100,
        num_channels=3,
        interleaved_samples=interleaved_samples,
    )
    assert waveform.num_channels == 3
    assert waveform.num_frames == 5
    assert waveform.frame_rate_hz == 44100
    #
    # Try fetching nonexistent values and receive a None response.
    assert waveform.get_sample(0, 3) is None
    assert waveform.get_sample(5, 0) is None
    assert waveform.get_sample(5, 3) is None
    #
    # Fetch every single value with get_sample() and verify it is correct.
    assert waveform.get_sample(0, 0) == np.float32(-1.0)
    assert waveform.get_sample(0, 1) == np.float32(-0.9)
    assert waveform.get_sample(0, 2) == np.float32(-0.8)
    #
    assert waveform.get_sample(1, 0) == np.float32(-0.7)
    assert waveform.get_sample(1, 1) == np.float32(-0.6)
    assert waveform.get_sample(1, 2) == np.float32(-0.5)
    #
    assert waveform.get_sample(2, 0) == np.float32(-0.4)
    assert waveform.get_sample(2, 1) == np.float32(-0.3)
    assert waveform.get_sample(2, 2) == np.float32(-0.2)
    #
    assert waveform.get_sample(3, 0) == np.float32(-0.1)
    assert waveform.get_sample(3, 1) == np.float32(0.0)
    assert waveform.get_sample(3, 2) == np.float32(0.1)
    #
    assert waveform.get_sample(4, 0) == np.float32(0.2)
    assert waveform.get_sample(4, 1) == np.float32(0.3)
    assert waveform.get_sample(4, 2) == np.float32(0.4)
