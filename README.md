# Babycat is an audio decoding and manipulation library

## Intro

### Babycat is written in Rust, with bindings to C, Python, and WebAssembly.

Write once. Run everywhere.

### Babycat is permissively-licensed.

Babycat is licensed under the MIT license, which allows you to use Babycat how you see fit.

### Babycat is battle-tested in industry.

Babycat was built at and is actively maintained by [Neocrym](https://www.neocrym.com/), a record label that use artificial intelligence to find and promote the world's greatest musicians. Neocrym uses Babycat to decode millions of songs as part of audio feature engineering pipelines for training machine learning models.

## Terminology

Babycat uses the [same audio terminology as the Apple Core Audio API](https://developer.apple.com/documentation/coreaudiotypes/audiostreambasicdescription?language=objc):

* An audio **stream** is a continuous series of data that represents a sound, such as a song.
* A **channel** is a discrete track of monophonic audio. A monophonic stream has one channel; a stereo stream has two channels.
* A **sample** is single number for a single audio channel in an audio stream.
* A **frame** is a collection of time-coincident samples. A frame has one sample for every channel.
* The **frame rate** (or **sample rate**) for a stream is the number of frames per second (hertz) of uncompressed audio.

## Acknowledgements

The first version of Babycat was an internal project at Neocrym written by [Ritik Neil Mishra](https://www.linkedin.com/in/ritikmishra). Since then, the code has been extended and open-sourced by [James Mishra](https://www.linkedin.com/in/jamesmishra).

Babycat is built on top of many high-quality open source packages, including:
* [Symphonia](https://github.com/pdeljanov/Symphonia) by Philip Deljanov
* [libsamplerate](http://www.mega-nerd.com/SRC/index.html) by Erik de Castro Lopo
* [Hound](https://github.com/ruuda/hound) by Ruud van Asseldonk
