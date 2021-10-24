babycat.batch
=============

.. toctree::
   :maxdepth: 2
   :hidden:

   .waveforms_from_files() <waveforms_from_files>
   .waveforms_from_files_to_numpy() <waveforms_from_files_to_numpy>


.. py:module:: babycat.batch

This submodule contains functions for decoding/demuxing multiple audio files in parallel.
Parallelism is achieved using multithreadinrg in Rust, which means that decoding will
not be slowed down by the  `Python Global Interpreter Lock (GIL) <https://realpython.com/python-gil/>`_.

Decoding audio
--------------
- :doc:`waveforms_from_files`: Uses multithreading in Rust to decode many audio files in parallel, returning :py:func:`~babycat.WaveformNamedResult` objects containing filenames, waveforms, and/or Python exceptions.
- :doc:`waveforms_from_files_to_numpy`: Uses multithreading in Rust to decode many audio files in parallel, directly returning NumPy arrays.
