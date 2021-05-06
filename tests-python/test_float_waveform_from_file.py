"""
Tests loading waveform from file.

These tests mirror the ones in ``../tests/test_float_waveform_from_file.rs``
"""
import pytest
from fixtures import *

import babycat

FloatWaveform = babycat.FloatWaveform
bexc = babycat.exceptions


def test_circus_of_freaks_default_1():
    waveform = FloatWaveform.from_file(COF_FILENAME)
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_wrong_time_offset_1():
    with pytest.raises(bexc.WrongTimeOffset):
        FloatWaveform.from_file(
            COF_FILENAME,
            start_time_milliseconds=1000,
            end_time_milliseconds=999,
        )


def test_circus_of_freaks_wrong_time_offset_2():
    with pytest.raises(bexc.WrongTimeOffset):
        FloatWaveform.from_file(
            COF_FILENAME,
            start_time_milliseconds=1000,
            end_time_milliseconds=1000,
        )


def test_circus_of_freaks_invalid_end_time_milliseconds_zero_pad_ending_1():
    with pytest.raises(bexc.CannotZeroPadWithoutSpecifiedLength):
        FloatWaveform.from_file(
            COF_FILENAME,
            start_time_milliseconds=5,
            end_time_milliseconds=0,
            zero_pad_ending=True,
        )


def test_circus_of_freaks_get_channels_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        num_channels=1,
    )
    assert waveform.num_channels == 1
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_get_channels_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        num_channels=2,
    )
    assert waveform.num_channels == 2
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_get_channels_too_many_1():
    with pytest.raises(bexc.WrongNumChannels):
        FloatWaveform.from_file(
            COF_FILENAME,
            num_channels=3,
        )


def test_circus_of_freaks_convert_to_mono_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        num_channels=2,
        convert_to_mono=True,
    )
    assert waveform.num_channels == 1
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_convert_to_mono_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        convert_to_mono=True,
    )
    assert waveform.num_channels == 1
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_convert_to_mono_invalid_1():
    with pytest.raises(bexc.WrongNumChannelsAndMono):
        FloatWaveform.from_file(
            COF_FILENAME,
            num_channels=1,
            convert_to_mono=True,
        )


def test_circus_of_freaks_start_end_milliseconds_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=1,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 44
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=10,
        end_time_milliseconds=11,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 44
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_3():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=30000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_4():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=15000,
        end_time_milliseconds=45000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_5():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=30000,
        end_time_milliseconds=60000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1169928
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=1,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 44
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=10,
        end_time_milliseconds=11,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 44
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_3():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=30000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_4():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=15000,
        end_time_milliseconds=45000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_5():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=30000,
        end_time_milliseconds=60000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_6():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=60000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2646000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_7():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=90000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 3969000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_start_end_milliseconds_zero_pad_ending_8():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=30000,
        end_time_milliseconds=90000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2646000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_end_milliseconds_zero_pad_ending_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        end_time_milliseconds=90000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 3969000
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_circus_of_freaks_invalid_resample_1():
    with pytest.raises(bexc.WrongFrameRateRatio):
        FloatWaveform.from_file(
            COF_FILENAME,
            frame_rate_hz=1,
        )


def test_circus_of_freaks_invalid_resample_2():
    with pytest.raises(bexc.WrongFrameRateRatio):
        FloatWaveform.from_file(
            COF_FILENAME,
            frame_rate_hz=20,
        )


def test_circus_of_freaks_invalid_resample_3():
    with pytest.raises(bexc.WrongFrameRateRatio):
        FloatWaveform.from_file(
            COF_FILENAME,
            frame_rate_hz=172,
        )


def test_circus_of_freaks_resample_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=22050,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1246464
    assert waveform.frame_rate_hz == 22050


def test_circus_of_freaks_resample_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=11025,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 623232
    assert waveform.frame_rate_hz == 11025


def test_circus_of_freaks_resample_3():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=88200,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 4985856
    assert waveform.frame_rate_hz == 88200


def test_circus_of_freaks_resample_4():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=4410,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 249293
    assert waveform.frame_rate_hz == 4410


def test_circus_of_freaks_resample_5():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=44099,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2492872
    assert waveform.frame_rate_hz == 44099


def test_circus_of_freaks_resample_6():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=48000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2713392
    assert waveform.frame_rate_hz == 48000


def test_circus_of_freaks_resample_7():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=60000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 3391739
    assert waveform.frame_rate_hz == 60000


def test_circus_of_freaks_resample_8():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=88200,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 4985856
    assert waveform.frame_rate_hz == 88200


def test_circus_of_freaks_resample_9():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=96000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 5426783
    assert waveform.frame_rate_hz == 96000


def test_circus_of_freaks_resample_10():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=200,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 11306
    assert waveform.frame_rate_hz == 200


def test_circus_of_freaks_resample_11():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=2000,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 113058
    assert waveform.frame_rate_hz == 2000


def test_circus_of_freaks_resample_12():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        frame_rate_hz=173,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 9780
    assert waveform.frame_rate_hz == 173


def test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_1():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=60000,
        frame_rate_hz=48000,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2880000
    assert waveform.frame_rate_hz == 48000


def test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_2():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=60000,
        frame_rate_hz=44099,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 2645940
    assert waveform.frame_rate_hz == 44099


def test_circus_of_freaks_start_end_milliseconds_resample_zero_pad_ending_3():
    waveform = FloatWaveform.from_file(
        COF_FILENAME,
        start_time_milliseconds=0,
        end_time_milliseconds=60000,
        frame_rate_hz=22050,
        zero_pad_ending=True,
    )
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == 1323000
    assert waveform.frame_rate_hz == 22050
