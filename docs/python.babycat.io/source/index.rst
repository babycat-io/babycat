.. toctree::
   :maxdepth: 3
   :hidden:

   Home <self>
   babycat/FloatWaveform/index.rst
   babycat/FloatWaveformNamedResult.rst
   babycat/exceptions.rst

.. toctree::
   :maxdepth: 3
   :hidden:
   :caption: External Links

   babycat.io <https://babycat.io>
   Babycat on GitHub <https://github.com/babycat-io/babycat>
   Technology at Neocrym <https://technology.neocrym.com/>
   Neocrym.com <https://www.neocrym.com>
   Privacy Policy <https://www.neocrym.com/privacy/>


.. raw:: html

    <div style="margin-top: 6em"></div>

    <h1 class="mega-header centered">
      Babycat Python documentation
    </h1>


Installation
-------------
Babycat's Python bindings requires Python 3.6 or newer and NumPy 1.16 or newer.

Babycat is `available on PyPI <https://pypi.org/project/babycat>`_.
You can install it by running the command:

.. code:: bash

   python3 -m pip install babycat

If Python is not your cup of tea, Babycat also has bindings for C, Rust, and WebAssembly.


API Reference
-------------

* :doc:`babycat/FloatWaveform/index` -- The main class for audio manipulation. **Start here** if you are new to Babycat.

* :doc:`babycat/FloatWaveformNamedResult` -- A Python type returned by batch audio decoders, used for storing waveforms or decoding errors.

* :doc:`babycat/exceptions` -- All exceptions defined by Babycat.


Usage
-----

If you want to learn the Babycat Python API in detail, start by looking at the documentation for :py:class:`babycat.FloatWaveform`.

Here is an example Python program that decodes and transforms a batch of audio files in parallel, creating NumPy arrays for the resulting waveforms.

.. code:: python

   from babycat import FloatWaveform

   # These are test files in the Babycat Git repository.
   filenames = [
      "audio-for-tests/andreas-theme/track.mp3",
      "audio-for-tests/blippy-trance/track.mp3",
      "audio-for-tests/voxel-revolution/track.mp3",
   ]

   # Decode the filenames in parallel, releasing the Python GIL.
   batch = FloatWaveform.from_many_files(

      # Perform the following transformations on EACH track:
      filenames,

      #  - Upsample the audio to 48khz.
      frame_rate_hz=48_000,

      #  - Average all audio channels into a monophonic channel.
      convert_to_mono=True,

      #  - Only select the first 60 seconds of audio.
      end_time_milliseconds=60_000,

      #  - If a track is shorter than 60 seconds, pad it with silence.
      zero_pad_ending=True,
   )

   # Iterate over the results.
   for result in batch:

      # Reraise any exceptions caught during decoding.
      if result.exception:
         raise result.exception

      # Now that you have a NumPy array for the waveform, do whatever you want.
      waveform_arr = result.waveform.numpy()
      print(result.name, waveform_arr)
