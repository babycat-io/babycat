.. toctree::
   :maxdepth: 3
   :hidden:

   Home <self>

.. toctree::
   :maxdepth: 3
   :hidden:
   :caption: External Links

   babycat.io <https://babycat.io>
   Babycat on GitHub <https://github.com/babycat-io/babycat>
   Technology at Neocrym <https://technology.neocrym.com/>
   Neocrym.com <https://www.neocrym.com>
   Privacy Policy <https://www.neocrym.com/privacy/>

.. raw:: html

    <div style="margin-top: 6em"></div>

    <h1 class="mega-header centered">
      Babycat WebAssembly documentation is coming soon...
    </h1>

You can currently build Babycat WebAssembly bindings by cloning Babycat's Git repository.

.. code:: bash

    git clone https://github.com/babycat-io/babycat.git
    cd babycat
    make build-wasm-bundler

And then you can find the compiled output at the path ``target/wasm/bundler``.

The Makefile in the Babycat repository has commands for building three types of WebAssembly bindings:

- ``make build-wasm-bundler`` -- Outputs a JavaScript package than can be used with a bundler like Webpack.
- ``make build-wasm-nodejs`` -- Outputs a JavaScript package that uses CommonJS modules.
- ``make build-wasm-web`` -- Outputs JavaScript that can be directly used by a web browser as an ECMASCript (ES) module.

In order to run these commands, you will need GNU Make, a C compiler, Rust 1.53.0 or newer, and wasm-pack. The `Babycat README on GitHub contains more information <https://github.com/babycat-io/babycat#system-requirements-for-compiling-babycat>`_ about requirements to compile Babycat.
