"""
Python benchmark for the batch decoder.
"""
import babycat

FILENAMES = [
    "./audio-for-tests/andreas-theme/track.flac",
    "./audio-for-tests/blippy-trance/track.wav",
    "./audio-for-tests/circus-of-freaks/track.flac",
    "./audio-for-tests/left-channel-tone/track.flac",
    "./audio-for-tests/mono-dtmf-tones/track.flac",
    "./audio-for-tests/on-hold-for-you/track.flac",
    "./audio-for-tests/tone-missing-sounds/track.flac",
    "./audio-for-tests/voxel-revolution/track.flac",
]


def test_decoding_batch_misc(benchmark):
    def _fn():
        for named_result in babycat.batch.waveforms_from_files(FILENAMES):
            if named_result.waveform is None:
                raise named_result.exception

    benchmark.pedantic(_fn, warmup_rounds=2, rounds=10, iterations=2)


def test_decoding_batch_misc_to_numpy_arrays(benchmark):
    def _fn():
        for named_result in babycat.batch.waveforms_from_files_to_numpy_arrays(
            FILENAMES
        ):
            if named_result.array is None:
                raise named_result.exception

    benchmark.pedantic(_fn, warmup_rounds=2, rounds=10, iterations=2)
