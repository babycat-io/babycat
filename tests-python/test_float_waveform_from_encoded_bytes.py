"""
Tests loading waveform from file.

These tests mirror the ones in ``../tests/test_float_waveform_from_encoded_bytes.rs``
"""
import pytest
from fixtures import *

import babycat

FloatWaveform = babycat.FloatWaveform
bexc = babycat.exceptions


with open(COF_FILENAME, "rb") as fh:
    COF_BYTES = fh.read()


def test_circus_of_freaks_default_1():
    waveform = FloatWaveform.from_encoded_bytes(COF_BYTES)
    assert waveform.num_channels == COF_NUM_CHANNELS
    assert waveform.num_frames == COF_NUM_FRAMES
    assert waveform.frame_rate_hz == COF_FRAME_RATE_HZ


def test_invalid_bytes_1():
    with pytest.raises(bexc.UnknownInputEncoding):
        FloatWaveform.from_encoded_bytes(b"asdfasdfasdfe")
