Welcome to Babycat Python's documentation!
==========================================

.. toctree::
   :maxdepth: 2
   :caption: Contents:

.. currentmodule:: babycat

.. autoclass:: FloatWaveform

   :decoders: These Python ``staticmethods`` either decode audio into a
      :py:class:`FloatWaveform` or create a waveform from scratch.

   .. autosummary::

      from_frames_of_silence
      from_milliseconds_of_silence
      from_encoded_bytes
      from_file
      from_many_files

   :getters: On an initialized :py:class:`FloatWaveform` object,
      these properties and methods tell you aboout the waveform.

   .. autosummary::

      frame_rate_hz
      num_channels
      num_frames
      numpy

   :encoders: These methods turn the in-memory decoded waveform into an
      encoded representation, like an audio file.

   .. autosummary::

      to_wav_buffer
      to_wav_file

   .. automethod:: from_frames_of_silence

   .. automethod:: from_milliseconds_of_silence

   .. automethod:: from_encoded_bytes

   .. automethod:: from_file

   .. automethod:: from_many_files

   .. autoattribute:: frame_rate_hz

   .. autoattribute:: num_channels

   .. autoattribute:: num_frames

   .. automethod:: numpy

   .. automethod:: to_wav_buffer

   .. automethod:: to_wav_file

.. autoclass:: FloatWaveformNamedResult

   .. autoattribute:: name

   .. autoattribute:: waveform

   .. autoattribute:: exception


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
