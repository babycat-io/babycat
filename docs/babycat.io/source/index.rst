.. raw:: html

   <img src="_static/images/1x/wordmark-on-transparent-with-black-text.png" class="mega-hero-img" />

   <h1 class="mega-header centered">
      Babycat is a library for decoding and manipulating audio files.
   </h1>

   <h3 class="mega-header">
      Babycat is written in Rust, with bindings to Python, C, and WebAssembly.
   </h3>

   <p>Babycat is designed to help you manipulate audio, no matter what programming language you are using.</p>

   <h3 class="mega-header">
      Babycat was written and open-sourced at <a href="https://www.neocrym.com" class="muted-link">Neocrym</a>, where it is used to decode and analyze tens of millions of songs.
   </h3>

   <p>Babycat is designed to process a lot of very different audio files with speed and parallelism.</p>

   <p>Babycat currently supports demuxing/decoding MP3, FLAC, and WAV/PCM files into waveforms in memory, and then writing those waveforms back as WAV.</p>

   <h2 class="mega-header">
      Documentation and packages
   </h2>

   <p>All Babycat source code, tests, and documentation is hosted in a single repository at <a href="https://github.com/babycat-io/babycat" class="muted-link">github.com/babycat-io/babycat</a>.</p>

   <p>You can find online documentation and pre-compiled packages for each Babycat binding at the below locations.</p>

   <table class="bigtable">
      <thead>
         <tr>
            <td></td>
            <td class="thead">Documentation</td>
            <td class="thead">Package Repository</td>
         </tr>
      </thead>
      <tbody>
         <tr>
            <td class="thead">Rust</td>
            <td><a href="https://rust.babycat.io" class="muted-link">rust.babycat.io</a></td>
            <td><a href="https://crates.io/crates/babycat" class="muted-link">crates.io/crates/babycat</a></td>
         </tr>
         <tr>
            <td class="thead">Python</td>
            <td><a href="https://python.babycat.io" class="muted-link">python.babycat.io</a></td>
            <td><a href="https://pypi.org/project/babycat" class="muted-link">pypi.org/project/babycat</a></td>
         </tr>
         <tr>
            <td class="thead">WebAssembly</td>
            <td><a href="https://wasm.babycat.io" class="muted-link">wasm.babycat.io</a></td>
            <td><a href="https://www.npmjs.com/package/babycat" class="muted-link">npmjs.com/package/babycat</a></td>
         </tr>
         <tr>
            <td class="thead">C</td>
            <td><a href="https://c.babycat.io" class="muted-link">c.babycat.io</a></td>
         </tr>
         <tr>
            <td class="thead">CLI</td>
            <td><a href="https://cli.babycat.io" class="muted-link">cli.babycat.io</a></td>
         </tr>
      </tbody>
   </table>


   <h2 class="mega-header">
      Examples
   </h2>

   <p>The above documentation links have more information about how to use Babycat, but here are a few examples of how to use Babycat in each of the supported languages.</p>

   <h3 class="mega-header">
      Decoding an audio file into a waveform.
   </h3>


This is an example of taking an audio file on disk and returning the waveform in memory.


.. tab:: Python

   .. code:: python

      from babycat import FloatWaveform

      waveform = FloatWaveform.from_file("audio.mp3")

.. tab:: Rust

   .. code:: rust

      use babycat::{DecodeArgs, FloatWaveform, Waveform};

      fn main() {
         let decode_args: DecodeArgs = Default::default();
         let waveform = FloatWaveform::from_file("audio.mp3", decode_args).unwrap();
      }


.. raw:: html

   <h2 class="mega-header">
      Acknowledgements
   </h2>

   <p>The first version of Babycat was an internal project at Neocrym written by <a href="https://www.linkedin.com/in/ritikmishra">Ritik Mishra</a>.
   Since then, the code has been extended and open-sourced by <a href="https://www.linkedin.com/in/jamesmishra">James Mishra</a>.</p>

   <p>Babycat is built on top of <em>many</em> high-quality open source packages, including:
      <ul>
         <li><a href="https://github.com/pdeljanov/Symphonia">Symphonia</a> by Philip Deljanov</li>
         <li><a href="http://www.mega-nerd.com/SRC/index.html">libsamplerate</a> by Erik de Castro Lopo</li>
         <li><a href="https://github.com/ruuda/hound">Hound</a> by Ruud van Asseldonk</li>
      </ul>
   </p>