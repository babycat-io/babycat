:hide-toc:

.. toctree::
   :maxdepth: 5
   :hidden:

   Home <self>


.. toctree::
   :maxdepth: 5
   :hidden:
   :caption: API Documentation

   Rust Docs (docs.rs) <https://docs.rs/babycat>
   Python Docs <api/python/index>
   WebAssembly Docs <api/wasm/index>
   C Docs <api/c/index>


.. toctree::
   :maxdepth: 5
   :hidden:
   :caption: Tutorials

   Using Babycat <tutorials/using-babycat/index>
   Contributing to Babycat <tutorials/contributing/index>


.. toctree::
   :maxdepth: 5
   :hidden:
   :caption: Releases

   Babycat on GitHub <https://github.com/babycat-io/babycat>
   Rust (crates.io) <https://crates.io/crates/babycat>
   Python (pypi.org) <https://pypi.org/project/babycat/>
   WebAssembly (npmjs.org) <https://www.npmjs.com/package/babycat>


.. raw:: html

   <img src="https://static.neocrym.com/images/babycat/v1/SVG/babycat-body-icon-transparent-white-text-social-media-cover.svg" class="mega-hero-img only-dark" />


Babycat is a library for decoding and manipulating audio files
==============================================================

Babycat is written in Rust, with bindings to Python, C, and WebAssembly
-----------------------------------------------------------------------
Babycat is designed to help you manipulate audio, no matter what programming language you are using

Babycat is written and open-sourced at `Neocrym <https://www.neocrym.com>`_
---------------------------------------------------------------------------
Neocrym is a record label that uses artificial intelligence to find and promote the world's greatest musicians.  At Neocrym, we use Babycat to decode and analyze tens of millions of songs.

Decode MP3, FLAC, and WAV
--------------------------
Babycat currently supports demuxing/decoding MP3, FLAC,and WAV/PCM files into waveforms in memory, and then writing those waveforms back as WAV.

Documentation and packages
--------------------------

All Babycat source code, tests, and documentation are hosted in a single repository at `github.com/babycat-io/babycat <https://github.com/babycat-io/babycat>`_.

You can find online documentation and pre-compiled packages for each Babycat binding at the below locations:

Examples
--------

The above documentation links have more information about how to use Babycat, but here are a few examples of how to use Babycat in each of the supported languages

Decoding an audio file into a waveform
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

This is an example of taking an audio file on disk and returning the waveform in memory.

This is an example of decoding a file named  ``'audio.mp3'``  into memory and then
printing:

- the number of frames in the audio
- the number of channels
- the frame rate

.. tab:: Python

   .. code:: python

      #!/usr/bin/env python3
      import babycat


      def main():
         try:
            waveform = babycat.FloatWaveform.from_file("audio.mp3")
         except (FileNotFoundError, babycat.exceptions.BabycatError) as exc:
            print("Decoding error:", exc)
            return
         print(
            f"Decoded {waveform.num_frames} frames with "
            f"{waveform.num_channels} channels at "
            f"{waveform.frame_rate_hz} hz"
         )


      if __name__ == "__main__":
         main()


.. tab:: Rust

   .. code:: rust

      use babycat::{DecodeArgs, FloatWaveform, Waveform};

      fn main() {
         let decode_args = DecodeArgs {
            ..Default::default()
         };
         let waveform = match FloatWaveform::from_file("audio.mp3", decode_args) {
            Ok(w) => w,
            Err(err) => {
                  println!("Decoding error: {}", err);
                  return;
            }
         };
         println!(
            "Decoded {} frames with {} channels at {} hz",
            waveform.num_frames(),
            waveform.num_channels(),
            waveform.frame_rate_hz(),
         );
      }


.. tab:: WebAssembly (Web)

   .. code:: javascript

      // In a web application, you can read an audio file using an
      // <input type="file" /> DOM node.
      // Here is an example of creating an input node and reading from it.

      import { FloatWaveform } from "babycat";

      function babycatDecode(arrayBuffer) {
         const arr = new Uint8Array(arrayBuffer);
         const waveform = FloatWaveform.fromEncodedArray(arr, {});
         console.log("Decoded",
            waveform.numFrames(),
            "frames with",
            waveform.numChannels(),
            "at",
            waveform.frameRateHz(),
            "hz"
         );
      }

      function handleFileUpload() {
         this.files[0].arrayBuffer().then((arrayBuffer) => babycatDecode(arrayBuffer));
      }

      function createFileDialog() {
         const fileUploader = document.createElement("input");
         fileUploader.type = "file";
         fileUploader.id = "fileUploader";
         fileUploader.addEventListener("change", handleFileUpload, false);

         return fileUploader;
      }

      document.body.appendChild(createFileDialog());


.. tab:: C

   .. code:: c

      #include <stdio.h>
      #include "babycat.h"


      int main() {
         babycat_DecodeArgs decode_args = babycat_init_default_decode_args();
         babycat_FloatWaveformResult waveform_result =
               babycat_float_waveform_from_file("audio.mp3", decode_args);
         if (waveform_result.error_num != 0) {
            printf("Decoding error: %u", waveform_result.error_num);
            return 1;
         }
         struct babycat_FloatWaveform *waveform = waveform_result.result;
         uint32_t num_frames = babycat_float_waveform_get_num_frames(waveform);
         uint32_t num_channels = babycat_float_waveform_get_num_channels(waveform);
         uint32_t frame_rate_hz = babycat_float_waveform_get_frame_rate_hz(waveform);
         printf("Decoded %u frames with %u channels at %u hz\n", num_frames,
                  num_channels, frame_rate_hz);

         return 0;
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
