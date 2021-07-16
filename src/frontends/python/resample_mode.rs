use pyo3::prelude::*;

/// Creates the `babycat.resample_mode` submodule, which is used to
/// store constants pointing to resampler backends.
pub fn make_resample_mode_submodule(py: Python) -> PyResult<&PyModule> {
    let resample_mode_submodule = PyModule::new(py, "resample_mode")?;

    resample_mode_submodule.setattr(
        "__doc__",
        "
A Python submodule to hold constants representing different resampler backends.

Babycat comes with different backends for resampling audio waveforms to
different frame rates. For example, CD audio is typically sampled at
44,100 hz and DVD audio is sampled at 48,000 hz. It is a common operation
to resample audio from one frame rate to another.

Babycat's Python bindings are typically compiled with support for the
following resamplers:

- :py:attr:`RESAMPLE_MODE_LIBSAMPLERATE`: This uses
  `libsamplerate <http://www.mega-nerd.com/SRC/>`_ at the
  ``SRC_SINC_BEST_QUALITY``. This backend produces the highest-quality
  output audio, but is often slightly slower than the other backends.
  The libsamplerate backend is always available in Babycat's Python
  bindings, but is currently not available in Babycat's WebAssembly bindings.

- :py:attr:`RESAMPLE_MODE_BABYCAT_LANCZOS`: This is a simple implementation
  of a `Lanczos resampler <https://en.wikipedia.org/wiki/Lanczos_resampling>`_.
  This is the fastest (and lowest-quality) resampler available in Babycat.

- :py:attr:`RESAMPLE_MODE_BABYCAT_SINC`: This is an implementation of
  a sinc resampler `as described by Stanford professor Julius O. Smith
  <https://ccrma.stanford.edu/~jos/resample/>`_. The speeed and quality
  of this resampler is in between the above two.

Example:
    **Resample using the ``BABYCAT_SINC`` resampler.**

    >>> from babycat import FloatWaveform
    >>> from babycat.resample_mode import *
    >>>
    >>> waveform = FloatWaveform.from_frames_of_silence(
    ...     frame_rate_hz=44100,
    ...     num_channels=2,
    ...     num_frames=1000,
    ... )
    >>> waveform
    <babycat.FloatWaveform: 1000 frames, 2 channels, 44100 hz>
    >>> resampled = waveform.resample_by_mode(
    ...     frame_rate_hz=11025,
    ...     resample_mode=RESAMPLE_MODE_BABYCAT_SINC,
    ... )
    <babycat.FloatWaveform: 250 frames, 2 channels, 11025 hz>

",
    )?;

    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_LIBSAMPLERATE",
        crate::backend::RESAMPLE_MODE_LIBSAMPLERATE,
    )?;
    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_BABYCAT_LANCZOS",
        crate::backend::RESAMPLE_MODE_BABYCAT_LANCZOS,
    )?;
    resample_mode_submodule.setattr(
        "RESAMPLE_MODE_BABYCAT_SINC",
        crate::backend::RESAMPLE_MODE_BABYCAT_SINC,
    )?;

    Ok(resample_mode_submodule)
}
