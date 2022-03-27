"""
Tests batch-loading waveforms from files into NumPy arrays.
"""
from fixtures import *

import babycat

ALL_SAME_FILENAMES = [COF_FILENAME, COF_FILENAME, COF_FILENAME]


def test_all_same_file_1():
    batch = babycat.batch.waveforms_from_files_into_numpys(ALL_SAME_FILENAMES)
    for named_result in batch:
        assert named_result.exception is None
        assert named_result.array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_all_same_file_2():
    batch = babycat.batch.waveforms_from_files_into_numpys(
        ALL_SAME_FILENAMES, end_time_milliseconds=15000
    )
    for named_result in batch:
        assert named_result.exception is None
        array = named_result.array
        assert array.shape == (661500, COF_NUM_CHANNELS)


def test_all_same_file_single_threaded_1():
    batch = babycat.batch.waveforms_from_files_into_numpys(
        ALL_SAME_FILENAMES,
        num_workers=1,
    )
    for named_result in batch:
        assert named_result.exception is None
        assert named_result.array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_different_filenames_1():
    batch = babycat.batch.waveforms_from_files_into_numpys(ALL_FILENAMES)
    for i, named_result in enumerate(batch):
        assert named_result.exception is None
        assert named_result.array.shape == (ALL_NUM_FRAMES[i], ALL_NUM_CHANNELS[i])


def test_file_not_found_error_1():
    batch = babycat.batch.waveforms_from_files_into_numpys([COF_FILENAME, "asdfasdf"])
    assert 2 == len(batch)
    assert batch[0].exception is None
    assert batch[0].array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)
    assert isinstance(batch[1].exception, FileNotFoundError)
