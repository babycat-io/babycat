babycat.batch
=============

.. py:module:: babycat.batch

This submodule contains functions for decoding/demuxing multiple audio files in parallel.
Parallelism is achieved using multithreadinrg in Rust, which means that decoding will
not be slowed down by the  `Python Global Interpreter Lock (GIL) <https://realpython.com/python-gil/>`_.

Decoding audio
--------------
.. toctree::
   :maxdepth: 2

   .waveforms_from_files() <waveforms_from_files>
