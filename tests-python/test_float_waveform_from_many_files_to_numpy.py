"""
Tests that we can directly decode NumPy arrays to waveforms.
"""
import pytest
from fixtures import *

from babycat import FloatWaveform

ALL_SAME_FILENAMES = [COF_FILENAME, COF_FILENAME, COF_FILENAME]


def test_empty():
    """Test from_many_files_to_numpy if we provide no filenames at all."""
    arrays = FloatWaveform.from_many_files_to_numpy([])
    assert len(arrays) == 0


def test_all_same_file_1():
    arrays = FloatWaveform.from_many_files_to_numpy(ALL_SAME_FILENAMES)
    assert len(arrays) == 3
    for arr in arrays:
        assert arr.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_all_same_file_2():
    arrays = FloatWaveform.from_many_files_to_numpy(
        ALL_SAME_FILENAMES,
        end_time_milliseconds=15000,
    )
    for arr in arrays:
        assert arr.shape == (661500, 2)


def test_different_filenames_1():
    arrays = FloatWaveform.from_many_files_to_numpy(ALL_FILENAMES)
    for i, arr in enumerate(arrays):
        assert arr.shape == (ALL_NUM_FRAMES[i], ALL_NUM_CHANNELS[i])


def test_file_not_found_error_1():
    with pytest.raises(FileNotFoundError):
        FloatWaveform.from_many_files_to_numpy([COF_FILENAME, "asdfadsfdas"])
