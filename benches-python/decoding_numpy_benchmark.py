"""
Runs Python benchmarks that measure the fastest way to turn a batch of audio files
into a Python list of NumPy arrays.

Our Rust code is pretty fast, but sometimes we encounter significant slowdowns
if we spend too much time trying to acquire the Python Global Interpreter Lock (GIL).

Our function :py:func:`~babycat.batch.from_files_to_numpy` manages to create NumPy
arrays without needing as much of the GIL as :py:func:`~babycat.batch.from_files`
seems to.

However, the  :py:func:`~babycat.batch.from_files_to_numpy` currently has a
worse error-handling situation and is currently not recommended to be used
against audio files where errors may be common.
"""
import unittest

import pytest

import babycat

assert_equal = unittest.TestCase().assertEqual

mark_benchmark = pytest.mark.benchmark(min_rounds=8, warmup=True, warmup_iterations=2)

NUM_FILENAMES = 12

FILENAMES = [
    "./audio-for-tests/andreas-theme/track.mp3",
    "./audio-for-tests/blippy-trance/track.mp3",
    "./audio-for-tests/circus-of-freaks/track.mp3",
    "./audio-for-tests/left-channel-tone/track.mp3",
    "./audio-for-tests/mono-dtmf-tones/track.mp3",
    "./audio-for-tests/on-hold-for-you/track.mp3",
    "./audio-for-tests/tone-missing-sounds/track.mp3",
    "./audio-for-tests/voxel-revolution/track.mp3",
] * NUM_FILENAMES


def assert_decoded(waveforms):
    """Assert the output shapes of decoded waveforms."""
    assert_equal(waveforms[0].shape, (9586944, 2))
    assert_equal(waveforms[1].shape, (5293440, 2))
    assert_equal(waveforms[2].shape, (2491776, 2))
    assert_equal(waveforms[3].shape, (1324800, 2))
    assert_equal(waveforms[4].shape, (442368, 1))
    assert_equal(waveforms[5].shape, (9620352, 2))
    assert_equal(waveforms[6].shape, (1324800, 1))
    assert_equal(waveforms[7].shape, (5728896, 2))


def no_convert():
    """Decode waveforms into NumPy arrays by manipulating them in Python."""
    babycat.batch.waveforms_from_files(FILENAMES)


@mark_benchmark
def test_no_convert(benchmark):
    """Run the ``no_convert`` benchmark."""
    benchmark(no_convert)


def waveforms_from_files():
    """Decode waveforms into NumPy arrays by manipulating them in Python."""
    return [
        nwr.waveform.to_numpy() for nwr in babycat.batch.waveforms_from_files(FILENAMES)
    ]


@mark_benchmark
def test_waveforms_from_files(benchmark):
    """Run the ``waveforms_from_files`` benchmark as a pytest unit test."""
    waveforms = benchmark(waveforms_from_files)
    assert_decoded(waveforms)


def waveforms_from_files_to_numpy():
    """Decode waveforms into NumPy arrays by manipulating them within Rust."""
    return babycat.batch.waveforms_from_files_to_numpy(FILENAMES)


@mark_benchmark
def test_waveforms_from_files_to_numpy(benchmark):
    """Run the ``waveforms_from_files_to_numpy`` benchmark as a pytest unit test."""
    waveforms = benchmark(waveforms_from_files_to_numpy)
    assert_decoded(waveforms)
