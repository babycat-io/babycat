babycat.FloatWaveform
=====================

.. py:class:: babycat.FloatWaveform


Waveform properties
-------------------
.. toctree::
   :maxdepth: 2

   .frame_rate_hz <frame_rate_hz>
   .num_channels <num_channels>
   .num_frames <num_frames>


Generating waveforms from silence
---------------------------------
.. toctree::
   :maxdepth: 2

   .from_frames_of_silence() <from_frames_of_silence>
   .from_milliseconds_of_silence() <from_milliseconds_of_silence>


Importing already-decoded audio waveforms
-----------------------------------------
.. toctree::
   :maxdepth: 2

   .from_interleaved_samples() <from_interleaved_samples>
   .from_numpy() <from_numpy>


Decoding audio
--------------
.. toctree::
   :maxdepth: 2

   .from_encoded_bytes() <from_encoded_bytes>
   .from_file() <from_file>
   .from_many_files() <from_many_files>


Resampling audio
----------------
.. toctree::
   :maxdepth: 2

   .resample() <resample>
   .resample_by_mode() <resample_by_mode>


Exporting decoded audio
-----------------------
.. toctree::
   :maxdepth: 2

   .to_interleaved_samples() <to_interleaved_samples>
   .to_numpy() <to_numpy>


Encoding audio
--------------
.. toctree::
   :maxdepth: 2


   .to_wav_buffer() <to_wav_buffer>
   .to_wav_file() <to_wav_file>
