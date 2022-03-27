"""
Tests loading waveform from encoded bytes into NumPy arrays.
"""
import pytest
from fixtures import *

import babycat

Waveform = babycat.Waveform
bexc = babycat.exceptions


with open(COF_FILENAME, "rb") as fh:
    COF_BYTES = fh.read()


def test_circus_of_freaks_default_1():
    array = Waveform.from_encoded_bytes_into_numpy(COF_BYTES)
    assert array.shape == (COF_NUM_FRAMES, COF_NUM_CHANNELS)


def test_invalid_bytes_1():
    with pytest.raises(bexc.UnknownInputEncoding):
        Waveform.from_encoded_bytes_into_numpy(b"asdfasdfasdfe")
