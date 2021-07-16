Terminology
===========

Data types
^^^^^^^^^^
- An audio **stream** is a continuous series of data that represents a sound, such as a song.
- A **channel** is a discrete track of monophonic audio. A monophonic stream has one channel. A stereo stream has two channels. An audio stream that contains 5.1 surround sound will have five normal channels and one Low Frequency Enhancement (LFE) channel.
- A **sample** is single numerical value in a single audio channel in an audio stream.
- A **frame** is a collection of samples from the same point in time--one sample for each channel.
- The **frame rate** (or **sample rate**) for a stream is the number of frames per second (hertz) of uncompressed audio.

Babycat stores audio as a single array of **interleaved** samples from channels. A waveform with two channels (e.g. left and right) are stored as:
```
[L R L R L R...]
```
We give every sample a two-dimensional index--first by frame and then by channel.

Resampling
^^^^^^^^^^
Babycat can resample audio from a **source** frame rate to a **destination** frame rate. Babycat also comes with several different backends that implement resampling:

- ``RESAMPLE_MODE_LIBSAMPLERATE``: This uses `libsamplerate <http://www.mega-nerd.com/SRC/>`_ at its highest quality setting. However, libsamplerate is not available in Babycat's WebAssembly bindings.
- ``RESAMPLE_MODE_BABYCAT_LANCZOS``: This is Babycat's implementation of a `Lanczos resampler <https://en.wikipedia.org/wiki/Lanczos_resampling>`_. This is the fastest (and lowest-quality) resampler available in Babycat.
- ``RESAMPLE_MODE_BABYCAT_SINC``: This is Babycat's implementation of a sinc resampler as `as described by Stanford professor Julius O. Smith <https://ccrma.stanford.edu/~jos/resample/>`_. The speed and quality of this resampler is in between the above two.
