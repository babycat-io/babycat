FROM babycat/ubuntu-minimal AS base

ARG DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]

WORKDIR /host

ARG CURL_CMD="curl --fail --show-error --silent --location"

# Install the main set of packages from the Debian repo.
RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
    curl \
    gnupg2 \
    software-properties-common \
    && ${CURL_CMD} https://deb.nodesource.com/setup_14.x | bash - \
    # We enable all "source" repositories for Ubuntu.
    && sed -i 's/^# deb/deb/' /etc/apt/sources.list \
    # We add the deadsnakes PPA for different Python versions,
    # including the "source" repository.
    && echo 'deb http://ppa.launchpad.net/deadsnakes/ppa/ubuntu focal main' >> /etc/apt/sources.list \
    && echo 'deb-src http://ppa.launchpad.net/deadsnakes/ppa/ubuntu focal main'  >> /etc/apt/sources.list \
    # Trust the deadsnakes PPA Apt key.
    && apt-key adv --keyserver keyserver.ubuntu.com --recv-keys F23C5A6CF475977595C89F51BA6932366A755776 \
    && apt-get update \
    && apt-get install --yes --no-install-recommends \
    # Install Node.js from the Node.js PPA
    nodejs \
    # Install compilers.
    gcc \
    clang \
    # Install tools for compiling binaries that statically-link to musl.
    musl-tools \
    # These are libraries for manipulating with audio.
    # The Python package "pydub" needs these. Babycat does not.
    ffmpeg \
    libavcodec-extra \
    # libffi-dev is needed to build the Python cryptography package.
    libffi-dev \
    # Now we list the installable dependencies for Valgrind, which we compile below.
    libc6-dbg \
    # Install pip. All of the Python versions are going to share
    # this implementation.
    python3-pip \
    # And now we install different Python versions.
    # 3.6
    python3.6 \
    python3.6-dev \
    python3.6-dbg \
    python3.6-venv \
    # 3.7
    python3.7 \
    python3.7-dev \
    python3.7-dbg \
    python3.7-venv \
    # 3.8
    python3.8 \
    python3.8-dev \
    python3.8-dbg \
    python3.8-venv \
    # 3.9
    python3.9 \
    python3.9-dev \
    python3.9-dbg \
    python3.9-venv \
    # 3.10
    python3.10 \
    python3.10-dev \
    python3.10-dbg \
    python3.10-venv

# Compile and install Valgrind.
ENV VALGRIND_VERSION=3.16.1
ADD https://sourceware.org/pub/valgrind/valgrind-${VALGRIND_VERSION}.tar.bz2 valgrind.tar.bz2
RUN tar xf valgrind.tar.bz2 \
    && cd valgrind-${VALGRIND_VERSION} \
    && ./configure \
    && make \
    && make install \
    && cd .. \
    && rm -rf valgrind.tar.bz2 valgrind-${VALGRIND_VERSION}

# Install optional Rust tools.
RUN cargo install cargo-valgrind flamegraph

# Remove the "I have no name!" bash shell prompt.
RUN echo 'export PS1="__@docker:\w \$ "' >> /etc/bash.bashrc
