"""
Test loading audio waveforms directly as NumPy arrays.

This test suite is specific to Babycat's Python bindings.
"""

from fixtures import *

import babycat

ALL_SAME_FILENAMES = [COF_FILENAME, COF_FILENAME, COF_FILENAME]


def test_all_same_file_1():
    batch = babycat.batch.waveforms_from_files_to_numpy(ALL_SAME_FILENAMES)
    for arr in batch:
        assert arr.shape[0] == COF_NUM_FRAMES
        assert arr.shape[1] == COF_NUM_CHANNELS


def test_all_same_file_2():
    batch = babycat.batch.waveforms_from_files_to_numpy(
        ALL_SAME_FILENAMES, end_time_milliseconds=15000
    )
    for arr in batch:
        assert arr.shape[0] == 661500
        assert arr.shape[1] == COF_NUM_CHANNELS


def test_all_same_file_single_threaded_1():
    batch = babycat.batch.waveforms_from_files_to_numpy(
        ALL_SAME_FILENAMES,
        num_workers=1,
    )
    for arr in batch:
        assert arr.shape[0] == COF_NUM_FRAMES
        assert arr.shape[1] == COF_NUM_CHANNELS
