Babycat Python API Documentation
================================

.. toctree::
   :maxdepth: 5
   :hidden:

   build_info <build_info/index>
   batch <batch/index>
   Waveform <Waveform/index>
   WaveformNamedResult <WaveformNamedResult/index>
   NumPyNamedResult <NumPyNamedResult/index>
   exceptions <exceptions>
   resample_mode <resample_mode>

This page shows the public API of the Python ``babycat`` package.

Submodules
----------
- :doc:`batch/index`: Functions for batched multithreaded decoding of multiple audio files.
- :doc:`exceptions`: All Babycat Python exception classes.
- :doc:`resample_mode`: Named constants for each Babycat resampling model.

Classes
-------
- :doc:`Waveform/index`: Handles decoding audio into a 32-bit floating point waveform.
- :doc:`WaveformNamedResult/index`: A wrapper class that holds either a :doc:`Waveform/index` or a Python exception.
- :doc:`NumPyNamedResult/index`: A wrapper class that holds either a NumPy array or a Python exception.
