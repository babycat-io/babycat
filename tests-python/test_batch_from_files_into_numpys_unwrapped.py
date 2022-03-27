"""
Tests batch-loading waveforms from files into NumPy arrays.
"""
from fixtures import *

import babycat

ALL_SAME_FILENAMES = [COF_FILENAME, COF_FILENAME, COF_FILENAME]


def test_all_same_file_1():
    arrays = babycat.batch.waveforms_from_files_into_numpys_unwrapped(
        ALL_SAME_FILENAMES
    )
    for array in arrays:
        assert array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_all_same_file_2():
    arrays = babycat.batch.waveforms_from_files_into_numpys_unwrapped(
        ALL_SAME_FILENAMES, end_time_milliseconds=15000
    )
    for array in arrays:
        assert array.shape == (661500, COF_NUM_CHANNELS)


def test_all_same_file_single_threaded_1():
    arrays = babycat.batch.waveforms_from_files_into_numpys_unwrapped(
        ALL_SAME_FILENAMES,
        num_workers=1,
    )
    for array in arrays:
        assert array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_different_filenames_1():
    arrays = babycat.batch.waveforms_from_files_into_numpys_unwrapped(ALL_FILENAMES)
    for i, array in enumerate(arrays):
        assert array.shape == (ALL_NUM_FRAMES[i], ALL_NUM_CHANNELS[i])


# def test_file_not_found_error_1():
# Currently our bindings raise pyo3.PanicException.
# We'll figure out how to get them to raise the correct exception later.
# with pytest.raises(FileNotFoundError):
#    babycat.batch.waveforms_from_files_into_numpys_unwrapped([COF_FILENAME, "asdfasdf"])
