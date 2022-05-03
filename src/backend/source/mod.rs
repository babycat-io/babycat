//! Iterators over audio.

use std::fmt::Debug;

mod append;
mod append_zeros;
mod convert_to_mono;
mod gain;
mod prepend_zeros;
mod scale;
mod select_channels;
mod skip_frames;
mod sum;
mod take_frames;
mod waveform_source;

pub use append::Append;
pub use append_zeros::AppendZeros;
pub use convert_to_mono::ConvertToMono;
pub use gain::Gain;
pub use prepend_zeros::PrependZeros;
pub use scale::Scale;
pub use select_channels::SelectChannels;
pub use skip_frames::SkipFrames;
pub use sum::Sum;
pub use take_frames::TakeFrames;
pub use waveform_source::WaveformSource;

use crate::backend::Error;
use crate::backend::Signal;
use crate::backend::Waveform;

/// A sample iterator created by an audio decoder.
pub trait Source: Signal + Iterator<Item = f32> + Debug {
    /// Append one [`Source`] after another [`Source`].
    ///
    /// Both Sources are required to have the same frame rate and number
    /// of channels.
    ///
    /// # Examples
    ///
    /// ```
    /// use babycat::assertions::assert_debug;
    /// use babycat::{symphonia::SymphoniaDecoder, Source, Signal, Waveform};
    ///
    /// // Load the FIRST audio file as a source.
    /// let f1 = "audio-for-tests/circus-of-freaks/track.flac";
    /// let mut d1 = SymphoniaDecoder::from_file(f1).unwrap();
    /// let s1 = d1.begin().unwrap();
    /// assert_debug(
    ///     &s1,
    ///     "SymphoniaSource { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    ///
    /// // Load the SECOND audio file as a source.
    /// let f2 = "audio-for-tests/andreas-theme/track.flac";
    /// let mut d2 = SymphoniaDecoder::from_file(f2).unwrap();
    /// let s2 = d2.begin().unwrap();
    /// assert_debug(
    ///     &s2,
    ///     "SymphoniaSource { 9586415 frames,  2 channels,  44100 hz,  3m 37s 379ms }"
    /// );
    ///
    /// // Append the second audio file AFTER the first audio file.
    /// // The new length (in frames) is the sum of the two audio files' lengths.
    /// let s1_s2 = s1.append(s2).unwrap();
    ///
    /// // Test that it works.
    /// assert_debug(
    ///     &s1_s2,
    ///     "Append { 2491247 + 9586415 = 12077662 frames,  2 channels,  44100 hz,  4m 33s 869ms }"
    /// );
    /// ```
    #[inline]
    fn append<S2: Source + Sized>(self, second: S2) -> Result<Append<Self, S2>, Error>
    where
        Self: Sized,
    {
        Append::new(self, second)
    }

    /// Pad the *beginning* of the [`Source`] with silence.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// // Set up a 3-frame, 2-channel `Source`.
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, -0.5, 0.1, 0.1, 0.5, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Prepend 3 frames (6 samples) of silent zero values.
    /// let out = source.prepend_zeros(3);
    ///
    /// // Test that it works.
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     &out_samples,
    ///     &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -0.5, 0.1, 0.1, 0.5, 1.0],
    /// );
    /// ```
    #[inline]
    fn prepend_zeros(self, num_frames: usize) -> PrependZeros<Self>
    where
        Self: Sized,
    {
        PrependZeros::new(self, num_frames)
    }

    /// Pad the *end* of the [`Source`] with silence.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// // Set up a 3-frame, 2-channel `Source`.
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, -0.5, 0.1, 0.1, 0.5, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Append 3 frames (6 samples) of silent zero values.
    /// let out = source.append_zeros(3);
    ///
    /// // Test that it works.
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     &out_samples,
    ///     &[-1.0, -0.5, 0.1, 0.1, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    /// );
    /// ```
    #[inline]
    fn append_zeros(self, num_frames: usize) -> AppendZeros<Self>
    where
        Self: Sized,
    {
        AppendZeros::new(self, num_frames)
    }

    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// // Set up a 3-frame, 2-channel `Source`.
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, -0.5, 0.0, 0.0, 0.5, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Alter the audio's gain by -10 dbFS.
    /// let out = source.gain_dbfs(-10.0);
    ///
    /// // Test that it works.
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     &out_samples,
    ///     &[-0.31622776, -0.15811388, 0.0, 0.0, 0.15811388, 0.31622776],
    /// );
    /// ```
    #[inline]
    fn gain_dbfs(self, dbfs: f32) -> Gain<Self>
    where
        Self: Sized,
    {
        Gain::new(self, dbfs)
    }

    /// Multiply each sample by a constant factor.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// // Set up a 3-frame, 2-channel `Source`.
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, -0.5, 0.0, 0.0, 0.5, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Multiply each sample by 0.25.
    /// let out = source.scale(0.25);
    ///
    /// // Test that it works.
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     out_samples,
    ///     &[-0.25, -0.125, 0.0, 0.0, 0.125, 0.25],
    /// );
    /// ```
    #[inline]
    fn scale(self, constant: f32) -> Scale<Self>
    where
        Self: Sized,
    {
        Scale::new(self, constant)
    }

    /// Skip the first `n` frames.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, 1.0, -0.5, 0.5, -0.0, 0.0, -0.5, 0.5, -1.0, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Skip the first 2 frames in the `Source`.
    /// let out = source.skip_frames(2);
    ///
    /// // Test that it works.
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     out_samples,
    ///     &[-0.0, 0.0, -0.5, 0.5, -1.0, 1.0],
    /// );
    /// ```
    #[inline]
    fn skip_frames(self, num_frames: usize) -> SkipFrames<Self>
    where
        Self: Sized,
    {
        SkipFrames::new(self, num_frames)
    }

    #[inline]
    fn sum<S2: Source + Sized>(self, second: S2) -> Sum<Self, S2>
    where
        Self: Sized,
    {
        Sum::new(self, second, 0)
    }

    #[inline]
    fn sum_with_frame_offset<S2: Source + Sized>(
        self,
        second: S2,
        offset_frames: usize,
    ) -> Sum<Self, S2>
    where
        Self: Sized,
    {
        Sum::new(self, second, offset_frames)
    }

    /// Take the first `n` frames.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    ///
    /// let frame_rate_hz = 44100;
    /// let num_channels = 2;
    /// let inp = vec![-1.0, 1.0, -0.5, 0.5, -0.0, 0.0, -0.5, 0.5, -1.0, 1.0];
    /// let source = WaveformSource::from_interleaved_samples(
    ///     frame_rate_hz, num_channels, &inp
    /// );
    ///
    /// // Take the first 3 frames.
    /// let out = source.take_frames(3);
    /// let out_samples = out.collect_interleaved_samples();
    /// assert_eq!(
    ///     out_samples,
    ///     &[-1.0, 1.0, -0.5, 0.5, -0.0, 0.0],
    /// );
    /// ```
    #[inline]
    fn take_frames(self, num_frames: usize) -> TakeFrames<Self>
    where
        Self: Sized,
    {
        TakeFrames::new(self, num_frames)
    }

    /// Select an interval of frames between `[start_idx, end_idx)`
    /// (including `start_idx` and excluding `end_idx`).
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, WaveformSource};
    /// let inp = vec![-1.0, 1.0, -0.5, 0.5, -0.0, 0.0, -0.5, 0.5, -1.0, 1.0];
    ///
    ///
    /// // Select frames 1-3.
    /// let source = WaveformSource::from_interleaved_samples(44100, 2, &inp);
    /// let selected = source.select_frames(1, 3);
    /// let out = selected.collect_interleaved_samples();
    /// assert_eq!(out, &[-0.5, 0.5, -0.0, 0.0]);
    ///
    ///
    /// // Select the first 2 frames.
    /// let source = WaveformSource::from_interleaved_samples(44100, 2, &inp);
    /// let selected = source.select_frames(0, 2);
    /// let out = selected.collect_interleaved_samples();
    /// assert_eq!(out, &[-1.0, 1.0, -0.5, 0.5]);
    ///
    ///
    /// // Skip the first 2 frames and select the remainder.
    /// let source = WaveformSource::from_interleaved_samples(44100, 2, &inp);
    /// let selected = source.select_frames(2, 0);
    /// let out = selected.collect_interleaved_samples();
    /// assert_eq!(out, &[-0.0, 0.0, -0.5, 0.5, -1.0, 1.0]);
    ///
    ///
    /// // Select all of the frames.
    /// let source = WaveformSource::from_interleaved_samples(44100, 2, &inp);
    /// let selected = source.select_frames(0, 0);
    /// let out = selected.collect_interleaved_samples();
    /// assert_eq!(out, &[-1.0, 1.0, -0.5, 0.5, -0.0, 0.0, -0.5, 0.5, -1.0, 1.0]);
    /// ```
    #[inline]
    fn select_frames<'a>(self, start_frame_idx: usize, end_frame_idx: usize) -> Box<dyn Source + 'a>
    where
        Self: 'a + Sized,
    {
        match (start_frame_idx, end_frame_idx) {
            (0, 0) => Box::new(self),
            (_, 0) => Box::new(self.skip_frames(start_frame_idx)),
            (0, _) => Box::new(self.take_frames(end_frame_idx)),
            (_, _) => Box::new(
                self.skip_frames(start_frame_idx)
                    .take_frames(end_frame_idx.saturating_sub(start_frame_idx)),
            ),
        }
    }

    /// Select the first `n` channels.
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, Waveform};
    ///
    /// // Create a 4-frame, 3-channel `Source`.
    /// let frame_rate_hz: u32 = 44100;
    /// let num_channels: u16 = 3;
    /// let interleaved_samples = vec![-1.0, 1.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.5, 0.0, -1.0, 1.0, 0.0];
    /// let s = Waveform::new(frame_rate_hz, num_channels, interleaved_samples).into_source();
    ///
    /// // Select the first 2 channels out of a 3-channel `Source`.
    /// let output_samples = s.select_first_channels(2).collect_interleaved_samples();
    ///
    /// // Test that it works.
    /// assert_eq!(
    ///     output_samples,
    ///     &[-1.0, 1.0, -0.5, 0.5, -0.5, 0.5, -1.0, 1.0],
    /// );
    /// ```
    #[inline]
    fn select_first_channels(self, selected_num_channels: u16) -> SelectChannels<Self>
    where
        Self: Sized,
    {
        SelectChannels::new(self, selected_num_channels)
    }

    /// Average the samples in each channel to produce a 1-channel monophonic [`Source`].
    ///
    /// # Examples
    /// ```
    /// use babycat::{Source, Waveform};
    ///
    /// let interleaved_samples = vec![-1.0, 0.5, -0.5, 0.25, -0.25, 0.125];
    /// let s = Waveform::new(44100, 2, interleaved_samples).into_source();
    ///
    /// let output_samples = s.convert_to_mono().collect_interleaved_samples();
    ///
    /// assert_eq!(
    ///     output_samples,
    ///     &[-0.25, -0.125, -0.0625],
    /// );
    /// ```
    #[inline]
    fn convert_to_mono(self) -> ConvertToMono<Self>
    where
        Self: Sized,
    {
        ConvertToMono::new(self)
    }

    /// Return a [`Vec<f32>`](std::vec::Vec) of collected interleaved samples.
    #[inline]
    fn collect_interleaved_samples(self) -> Vec<f32>
    where
        Self: Sized,
    {
        self.collect()
    }

    /// Collect all samples into memory and return a [`Waveform`].
    ///
    /// # Examples
    /// ```
    /// use babycat::assertions::assert_debug;
    /// use babycat::{symphonia::SymphoniaDecoder, Source, Signal};
    ///
    /// // Decode an audio file into a `SymphoniaSource` iterator.
    /// let mut decoder = SymphoniaDecoder::from_file("audio-for-tests/circus-of-freaks/track.flac").unwrap();
    /// let source = decoder.begin().unwrap();
    /// assert_debug(
    ///     &source,
    ///     "SymphoniaSource { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    ///
    /// // Convert the `SymphoniaSource` iterator into a `Waveform` struct,
    /// // which loads all of the audio samples into memory.
    /// let waveform = source.to_waveform();
    /// assert_debug(
    ///     &waveform,
    ///     "Waveform { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    /// ```
    #[inline]
    fn to_waveform(self) -> Waveform
    where
        Self: Sized,
    {
        let frame_rate_hz = self.frame_rate_hz();
        let num_channels = self.num_channels();
        let interleaved_samples = self.collect_interleaved_samples();
        Waveform::new(frame_rate_hz, num_channels, interleaved_samples)
    }

    /// Collect all samples and return a [`WaveformSource`].
    ///
    /// # Examples
    /// ```
    /// use babycat::assertions::assert_debug;
    /// use babycat::{symphonia::SymphoniaDecoder, Source, Signal};
    ///
    /// // Decode an audio file into a `SymphoniaSource` iterator.
    /// let mut decoder = SymphoniaDecoder::from_file("audio-for-tests/circus-of-freaks/track.flac").unwrap();
    /// let source = decoder.begin().unwrap();
    /// assert_debug(
    ///     &source,
    ///     "SymphoniaSource { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    ///
    /// // Convert the `SymphoniaSource` iterator into a `WaveformSource` iterator,
    /// // which loads all of the audio samples into memory.
    /// let waveform_source = source.to_waveform_source();
    /// assert_debug(
    ///     &waveform_source,
    ///     "WaveformSource { 2491247 frames,  2 channels,  44100 hz,  56s 490ms }"
    /// );
    ///
    #[inline]
    fn to_waveform_source(self) -> WaveformSource
    where
        Self: Sized,
    {
        let waveform = self.to_waveform();
        waveform.into_source()
    }
}

impl Source for Box<dyn Source + '_> {}

impl Signal for Box<dyn Source + '_> {
    #[inline]
    fn frame_rate_hz(&self) -> u32 {
        (&**self).frame_rate_hz()
    }

    #[inline]
    fn num_channels(&self) -> u16 {
        (&**self).num_channels()
    }

    #[inline]
    fn num_frames_estimate(&self) -> Option<usize> {
        (&**self).num_frames_estimate()
    }
}
