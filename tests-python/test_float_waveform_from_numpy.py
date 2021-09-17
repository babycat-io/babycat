"""
Test loading waveforms from NumPy arrays.
"""
import numpy as np
import pytest
from fixtures import COF_FILENAME, COF_FRAME_RATE_HZ, COF_NUM_CHANNELS, COF_NUM_FRAMES

from babycat import FloatWaveform


def test_circus_of_freaks_default_1():
    w1 = FloatWaveform.from_file(COF_FILENAME)
    waveform = FloatWaveform.from_numpy(
        frame_rate_hz=w1.frame_rate_hz,
        arr=w1.to_numpy(),
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ
    np.testing.assert_array_equal(w1.to_numpy(), waveform.to_numpy())


def test_four_frames_three_channels():
    frame = np.array([-1.0, 0.0, 1.0], dtype="float32")
    arr = np.stack([frame, frame, frame, frame])
    waveform = FloatWaveform.from_numpy(
        frame_rate_hz=44_100,
        arr=arr,
    )
    assert waveform.num_channels == 3
    assert waveform.num_frames == 4
    assert waveform.frame_rate_hz == 44_100
    np.testing.assert_array_equal(arr, waveform.to_numpy())


def test_wrong_dtype_1():
    """Raise a TypeError when we pass a float64 array."""
    frame = np.array([-1.0, 0.0, 1.0], dtype="float64")
    arr = np.stack([frame, frame, frame])
    with pytest.raises(TypeError):
        FloatWaveform.from_numpy(
            frame_rate_hz=44_100,
            arr=arr,
        )


def test_wrong_dtype_2():
    """Raise a TypeError when we pass an integer array."""
    frame = np.array([-1, 0, 1], dtype="int64")
    arr = np.stack([frame, frame, frame])
    with pytest.raises(TypeError):
        FloatWaveform.from_numpy(
            frame_rate_hz=44_100,
            arr=arr,
        )


def test_wrong_shape_1():
    """Raise a TypeError when we pass a 1D NumPy array."""
    arr = np.array([-1.0, 0.0, 1.0, -1.0, 0.0, 1.0], dtype="float32")
    with pytest.raises(TypeError):
        FloatWaveform.from_numpy(frame_rate_hz=44_100, arr=arr)


def test_wrong_shape_2():
    """Raise a TypeError when we pass a 3D NumPy array."""
    arr = np.array(
        [
            [
                [-1.0, 1.0],
                [-1.0, 1.0],
            ],
            [
                [-1.0, 1.0],
                [-1.0, 1.0],
            ],
        ],
        dtype="float32",
    )
    with pytest.raises(TypeError):
        FloatWaveform.from_numpy(frame_rate_hz=44_100, arr=arr)
