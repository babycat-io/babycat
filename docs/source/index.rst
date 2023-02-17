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

   tutorials/terminology
   tutorials/development-requirements
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


Babycat is an open-source library for decoding and manipulating audio files
===========================================================================

Features
--------

Decoding, resampling, and encoding
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Babycat's core feature set includes:

- decoding MP3, FLAC, and WAV.
- resampling audio to different frame rates.
- encoding waveforms to WAV.

Bindings for Rust, C, Python, and JavaScript/WebAssembly
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Babycat can be used from the following target languages:

- **Rust**. The majority of Babycat is written in Rust, with the exception of a few C dependencies like  `libsamplerate <http://www.mega-nerd.com/SRC/index.html>`_. 
- **Python**. Babycat's Python bindings allow you to decode, resample, and encode audio without being slowed down by Python's Global Interpreter Lock (GIL). Babycat also integrates with Jupyter, allowing you to play and listen to audio streams decoded by Babycat inside of a Jupyter notebook.
- **WebAssembly**. Babycat generates JavaScript/WebAssembly bindings that can run either in a web browser or in Node.js.
- **C**. Babycat exposes a C API, which is useful for both creating audio analysis projects in C or creating Babycat bindings for languages not mentioned above.


Effective multithreading and parallelism
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Babycat is designed to parallelize the decoding of many audio files across multiple CPU cores. Babycat's Python bindings allow for parallel audio decoding without being slowed down by Python's Global Interpreter Lock.

Open source under the MIT license
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The audio ecosystem is full of expensive proprietary software packages, or (L)GPL-licensed code that restricts how you can use it. In contrast, Babycat is licensed under the MIT license, allowing you to use Babycat any way you want for free.

Babycat is battle-tested in industry
------------------------------------
Babycat was built at and is actively maintained by  `Neocrym <https://www.neocrym.com>`_, a record label that uses artificial intelligence to find and promote the world's greatest musicians. Neocrym uses Babycat to decode millions of songs as part of audio feature engineering pipelines for machine learning models.

Documentation and releases
--------------------------
You can find Babycat's source code at `github.com/babycat-io/babycat <https://github.com/babycat-io/babycat>`_.

This website is where we keep Babycat's documentation. You can find documentation and releases for each Babycat binding at the following locations:

.. raw:: html

   <table class="bigtable">
      <thead>
         <tr>
            <td><strong>Binding</strong></td>
            <td><strong>Documentation</strong></td>
            <td><strong>Releases</strong></td>
         </tr>
      </thead>
      <tbody>
         <tr>
            <td><strong>Rust</strong></td>
            <td><a href="https://docs.rs/babycat">docs.rs/babycat</a></td>
            <td><a href="https://crates.io/crates/babycat">crates.io/crates/babycat</a></td>
         </tr>
         <tr>
            <td><strong>Python</strong></td>
            <td><a href="https://babycat.io/api/python/">babycat.io/api/python</a></td>
            <td><a href="https://pypi.org/project/babycat/">pypi.org/project/babycat</a></td>
         </tr>
         <tr>
            <td><strong>WebAssembly</strong></td>
            <td><a href="https://babycat.io/api/wasm/">babycat.io/api/wasm</a></td>
            <td><a href="https://www.npmjs.com/package/babycat">npmjs.com/package/babycat</a></td>
         </tr>
         <tr>
            <td><strong>C</strong></td>
            <td><a href="https://babycat.io/api/c/">babycat.io/api/c</a></td>
            <td>No releases yet. You can compile from source.</td>
         </tr>
      </tbody>
   </table>


Examples
--------
The above documentation links have more information about how to use Babycat, but here are a few examples of how to use Babycat in each of the supported languages

Decoding an audio file into a waveform
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
This is an example of decoding a file named ``'audio.mp3'`` into memory and then printing:

- the number of frames in the audio
- the number of channels
- the frame rate

.. tab:: Python

   .. code:: python

      #!/usr/bin/env python3
      import babycat


      def main():
         try:
            waveform = babycat.Waveform.from_file("audio.mp3")
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

      use babycat::{Source, WaveformArgs, Waveform};

      fn main() {
         let waveform_args = WaveformArgs {
            ..Default::default()
         };
         let waveform = match Waveform::from_file("audio.mp3", waveform_args) {
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

      import { Waveform } from "babycat";

      function babycatDecode(arrayBuffer) {
         const arr = new Uint8Array(arrayBuffer);
         const waveform = Waveform.fromEncodedArray(arr, {});
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
         babycat_WaveformArgs waveform_args = babycat_waveform_args_init_default();
         babycat_WaveformResult waveform_result =
               babycat_waveform_from_file("audio.mp3", waveform_args);
         if (waveform_result.error_num != 0) {
            printf("Decoding error: %u", waveform_result.error_num);
            return 1;
         }
         struct babycat_Waveform *waveform = waveform_result.result;
         uint32_t num_frames = babycat_waveform_get_num_frames(waveform);
         uint32_t num_channels = babycat_waveform_get_num_channels(waveform);
         uint32_t frame_rate_hz = babycat_waveform_get_frame_rate_hz(waveform);
         printf("Decoded %u frames with %u channels at %u hz\n", num_frames,
                  num_channels, frame_rate_hz);

         return 0;
      }



Acknowledgements
----------------
The first version of Babycat was an internal project at Neocrym written by `Ritik Mishra <https://www.linkedin.com/in/ritikmishra>`_. Since then, the code has been extended and open-sourced by `James Mishra <https://www.linkedin.com/in/jamesmishra>`_.

Babycat is built on top of *many* high-quality open source packages, including:

- `Symphonia <https://github.com/pdeljanov/Symphonia>`_ for audio decoding.
- `libsamplerate <http://www.mega-nerd.com/SRC/index.html>`_ for high-quality audio resampling.
- `Hound <https://github.com/ruuda/hound>`_ for WAV encoding.
- `PyO3 <https://github.com/PyO3/pyo3>`_ for generating Python bindings.
- `cbindgen <https://github.com/eqrion/cbindgen>`_ for generating C bindings.
- `wasm-bindgen <https://github.com/rustwasm/wasm-bindgen>`_ for generating WebAssembly bindings.

Babycat's goal is to provide a simple and consistent API on top of the existing audio ecosystem, without sacrificing performance, portability, or permissive licensing.
