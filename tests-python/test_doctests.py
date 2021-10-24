"""Run docstring tests in the babycat module."""
from doctest import DocTestFinder, DocTestRunner

import babycat


def run_doctest(obj):
    """Run any doctests that are in `obj`'s docstring."""
    finder = DocTestFinder(verbose=False, recurse=False)
    runner = DocTestRunner(verbose=False)
    for found_test in finder.find(obj):
        results = runner.run(found_test)
        assert results.failed == 0


def test_waveform_from_frames_of_silence():
    run_doctest(babycat.Waveform.from_frames_of_silence)


def test_waveform_from_milliseconds_of_silence():
    run_doctest(babycat.Waveform.from_milliseconds_of_silence)


def test_waveform_from_encoded_bytes():
    run_doctest(babycat.Waveform.from_encoded_bytes)


def test_waveform_from_file():
    run_doctest(babycat.Waveform.from_file)


def test_batch():
    run_doctest(babycat.batch)


def test_batch_waveforms_from_files():
    run_doctest(babycat.batch.waveforms_from_files)


def test_batch_waveforms_from_files_to_numpy():
    run_doctest(babycat.batch.waveforms_from_files_to_numpy)
