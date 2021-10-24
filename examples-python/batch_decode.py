#!/usr/bin/env python3
import logging
import time

import babycat

FILENAMES = ["audio-for-tests/circus-of-freaks/track.mp3"] * 200

def waveforms_from_files():
    t1 = time.monotonic()
    results = babycat.batch.waveforms_from_files(
        filenames=FILENAMES
    )
    t2 = time.monotonic()
    for result in results:
        result.waveform.to_numpy()
    t3 = time.monotonic()
    print("waveforms_from_files t2:", t2 - t1)
    print("waveforms_from_files total:", t3-t1)


def waveforms_from_files_to_numpy():
    t1 = time.monotonic()
    results = babycat.batch.waveforms_from_files_to_numpy(
        filenames=FILENAMES
    )
    t2 = time.monotonic()
    t3 = time.monotonic()
    print("waveforms_from_files t2:", t2 - t1)
    print("waveforms_from_files total:", t3-t1)

def main():
    waveforms_from_files()
    waveforms_from_files_to_numpy()

if __name__ == "__main__":
    main()
