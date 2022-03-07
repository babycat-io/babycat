"""
Tests that all of the resamplers produce waveforms of the same dimension.

These tests mirror the ones in ``../tests/test_waveform_resample_method.rs``.
"""
from math import ceil

from fixtures import *

import babycat

Waveform = babycat.Waveform

RESAMPLE_MODES = {
    key: val
    for key, val in babycat.resample_mode.__dict__.items()
    if "RESAMPLE_MODE" in key
}


def decode_and_assert(
    *,
    filename: str,
    waveform_args: dict,
    frame_rate_hz: int,
    expected_num_channels: int,
    expected_num_frames: int,
    expected_frame_rate_hz: int,
):
    waveform = Waveform.from_file(
        filename,
        **waveform_args,
    )
    for mode_name, resample_mode in RESAMPLE_MODES.items():
        resampled = waveform.resample_by_mode(
            frame_rate_hz=frame_rate_hz,
            resample_mode=resample_mode,
        )
        error_msg = f"Failed with resample mode {mode_name}."
        assert resampled.num_channels == expected_num_channels, error_msg
        assert resampled.num_frames == expected_num_frames, error_msg
        assert resampled.frame_rate_hz == expected_frame_rate_hz, error_msg


def test_circus_of_freaks_no_change_1():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=COF_FRAME_RATE_HZ,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=COF_NUM_FRAMES,
        expected_frame_rate_hz=COF_FRAME_RATE_HZ,
    )


def test_circus_of_freaks_44099():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=44099,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=2491191,
        expected_frame_rate_hz=44099,
    )


def test_circus_of_freaks_44101():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=44101,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=2491304,
        expected_frame_rate_hz=44101,
    )


def test_circus_of_freaks_22050():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=22050,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=ceil(COF_NUM_FRAMES / 2),
        expected_frame_rate_hz=22050,
    )


def test_circus_of_freaks_11025():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=11025,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=ceil(COF_NUM_FRAMES / 4),
        expected_frame_rate_hz=11025,
    )


def test_circus_of_freaks_88200():
    decode_and_assert(
        filename=COF_FILENAME,
        waveform_args={},
        frame_rate_hz=88200,
        expected_num_channels=COF_NUM_CHANNELS,
        expected_num_frames=COF_NUM_FRAMES * 2,
        expected_frame_rate_hz=88200,
    )
