"""
Run benchmarks comparing FloatWaveform.from_many_files to FloatWaveform.from_many_files_to_numpy.

We want to make it as *fast* as possible to convert a list of audio files on disk
into a list of NumPy arrays in memory.
"""
from babycat import FloatWaveform

NUM_FILENAMES_1 = 48
FILENAMES_1 = ["./audio-for-tests/circus-of-freaks/track.mp3"] * NUM_FILENAMES_1


def from_many_files_no_iteration_1():
    """Decode audio files, but don't convert them into NumPy arrays."""
    FloatWaveform.from_many_files(FILENAMES_1)


def test_bench_from_many_files_no_iteration_1(benchmark):
    benchmark(from_many_files_no_iteration_1)


def from_many_files_1():
    """Decode audio files, but convert them to NumPy arrays on the Python side."""
    named_results = FloatWaveform.from_many_files(FILENAMES_1)
    for nr in named_results:
        nr.waveform.to_numpy()


def test_bench_from_many_files_1(benchmark):
    benchmark(from_many_files_1)


def from_many_files_to_numpy_1():
    """Decode audio files, but convert them into NumPy arrays on the Rust side."""
    FloatWaveform.from_many_files_to_numpy(FILENAMES_1)


def test_bench_from_many_files_to_numpy_1(benchmark):
    benchmark(from_many_files_to_numpy_1)
