"""
Tests loading waveform from file.

These tests mirror the ones in ``../tests/test_waveform_batch_from_files.rs``
"""
from fixtures import *

import babycat

ALL_SAME_FILENAMES = [COF_FILENAME, COF_FILENAME, COF_FILENAME]


def test_all_same_file_1():
    batch = babycat.batch.waveforms_from_files(ALL_SAME_FILENAMES)
    for named_result in batch:
        assert named_result.exception is None
        waveform = named_result.waveform
        assert waveform.num_channels == COF_NUM_CHANNELS
        assert waveform.num_frames == COF_NUM_FRAMES
        assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_all_same_file_2():
    batch = babycat.batch.waveforms_from_files(
        ALL_SAME_FILENAMES, end_time_milliseconds=15000
    )
    for named_result in batch:
        assert named_result.exception is None
        waveform = named_result.waveform
        assert waveform.num_channels == COF_NUM_CHANNELS
        assert waveform.num_frames == 661500
        assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_all_same_file_single_threaded_1():
    batch = babycat.batch.waveforms_from_files(
        ALL_SAME_FILENAMES,
        num_workers=1,
    )
    for named_result in batch:
        assert named_result.exception is None
        waveform = named_result.waveform
        assert waveform.num_channels == COF_NUM_CHANNELS
        assert waveform.num_frames == COF_NUM_FRAMES
        assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_different_filenames_1():
    batch = babycat.batch.waveforms_from_files(ALL_FILENAMES)
    for i, named_result in enumerate(batch):
        assert named_result.exception is None
        waveform = named_result.waveform
        assert ALL_NUM_CHANNELS[i] == waveform.num_channels
        assert ALL_NUM_FRAMES[i] == waveform.num_frames
        assert ALL_FRAME_RATE_HZ[i] == waveform.frame_rate_hz


def test_file_not_found_error_1():
    batch = babycat.batch.waveforms_from_files([COF_FILENAME, "asdfasdf"])
    assert 2 == len(batch)
    assert batch[0].exception is None
    assert batch[0].waveform.num_channels == COF_NUM_CHANNELS
    assert batch[0].waveform.num_frames == COF_NUM_FRAMES
    assert batch[0].waveform.frame_rate_hz == COF_FRAME_RATE_HZ
    assert batch[1].waveform is None
    assert isinstance(batch[1].exception, FileNotFoundError)
