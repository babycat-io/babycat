"""
Tests loading waveform from file into NumPy arrays.
"""
import pytest
from fixtures import *

import babycat

Waveform = babycat.Waveform
bexc = babycat.exceptions


def test_circus_of_freaks_default_1():
    array = Waveform.from_file_into_numpy(COF_FILENAME)
    assert array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_circus_of_freaks_wrong_time_offset_1():
    with pytest.raises(bexc.WrongTimeOffset):
        Waveform.from_file_into_numpy(
            COF_FILENAME,
            start_time_milliseconds=1000,
            end_time_milliseconds=999,
        )


def test_file_not_found_error():
    with pytest.raises(FileNotFoundError):
        Waveform.from_file_into_numpy(
            "file-not-found",
            start_time_milliseconds=1000,
            end_time_milliseconds=999,
        )
