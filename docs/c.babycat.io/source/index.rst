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
      Babycat C documentation is coming soon...
    </h1>

Babycat's C header file comes with inline documentation. You can clone Babycat and generate the C headers using the following commands:

.. code:: bash

    git clone https://github.com/babycat-io/babycat.git
    cd babycat
    make babycat.h

Then, you can build the actual Babycat C library using the following command:

.. code:: bash

    cargo build --release --no-default-features --features=frontend-c
   
and the resulting library will be built in the ``target/release`` subdirectory.

In order to run these commands, you will need GNU Make, a C compiler, Rust 1.50.0 or newer, and cbindgen. The `Babycat README on GitHub contains more information <https://github.com/babycat-io/babycat#system-requirements-for-compiling-babycat>`_ about requirements to compile Babycat.
