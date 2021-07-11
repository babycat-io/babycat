FloatWaveform
=============

.. currentmodule:: babycat

.. toctree::
   :maxdepth: 1
   :hidden:

   from_encoded_bytes <from_encoded_bytes.rst>
   from_file <from_file.rst>
   from_many_files <from_many_files.rst>
   from_frames_of_silence <from_frames_of_silence.rst>
   from_milliseconds_of_silence <from_milliseconds_of_silence.rst>
   resample <resample.rst>
   resample_by_mode <resample_by_mode.rst>
   to_wav_buffer <to_wav_buffer.rst>
   to_wav_file <to_wav_file.rst>


:py:class:`FloatWaveform` is the main class used for audio manipulation in Babycat. It typically contains a buffer of audio samples--each stored as a 32-bit floating point number.

You'll probably want to create a :py:class:`FloatWaveform` by either (1) decoding an audio file or (2) creating a waveform from scratch.

At the end of your workflow, you can encode the waveform back into a playable audio file and save it to the filesystem.

Decoding audio files
--------------------

These Python ``staticmethods`` can decode audio files in memory or on the filesystem.

* :py:meth:`FloatWaveform.from_encoded_bytes`: Decodes audio stored as :py:class:`bytes`.

* :py:meth:`FloatWaveform.from_file`: Decodes audio stored in a file.

* :py:meth:`FloatWaveform.from_many_files`: Uses multithreading in Rust to decode many audio files in parallel. This returns a *list* of :py:class:`FloatWaveformNamedResult` objects, each of which contain either a :py:class:`FloatWaveform` from a successful decoding or a Python exception resulting from a failed decoding.

Creating waveforms from scratch
-------------------------------

These Python ``staticmethods`` create brand-new (silent) waveforms for further manipulation.

* :py:meth:`FloatWaveform.from_frames_of_silence`: Create a silent waveform by specifying its length in frames.

* :py:meth:`FloatWaveform.from_milliseconds_of_silence`: Create a silent waveform by specifying its length in milliseconds.

Modifying waveforms in memory
-----------------------------

Once you create a :py:class:`FloatWaveform` object, you can create new, modified versions of the waveform by calling these methods:

* :py:meth:`FloatWaveform.resample`: Resamples the waveform to a different frame rate, using the default resampler.

* :py:meth:`FloatWaveform.resample_by_mode`: Resamples the waveform using the resampler of your choice.

Encoding waveforms and saving to the filesystem
-----------------------------------------------

* :py:meth:`FloatWaveform.to_wav_buffer`: Returns the waveform as a WAV-encoded :py:class:`bytearray`.

* :py:meth:`FloatWaveform.to_wav_file`: Saves the waveform as a WAV file on the filesystem.


FloatWaveform getters and setters
---------------------------------

.. autoclass:: FloatWaveform

   .. autosummary::

      frame_rate_hz
      num_channels
      num_frames
      numpy

   .. autoattribute:: frame_rate_hz

   .. autoattribute:: num_channels

   .. autoattribute:: num_frames

   .. automethod:: numpy
