// Symphonia
// Copyright (c) 2019 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

pub mod default {
    //! The `default` module provides common convenience functions to get an implementer
    //! up-and-running as quickly as possible, and to reduce boiler-plate. Using the `default` module
    //! is completely optional and incurs no overhead unless actually used.

    use lazy_static::lazy_static;

    use symphonia_core::probe::Probe;
    use symphonia_core::codecs::CodecRegistry;

    lazy_static! {
        static ref CODEC_REGISTRY: CodecRegistry = {
            #[cfg(feature = "aac")]
            use symphonia_codec_aac::AacDecoder;
            #[cfg(feature = "flac")]
            use symphonia_bundle_flac::FlacDecoder;
            #[cfg(feature = "mp3")]
            use symphonia_bundle_mp3::Mp3Decoder;
            #[cfg(feature = "pcm")]
            use symphonia_codec_pcm::PcmDecoder;

            let mut registry = CodecRegistry::new();

            #[cfg(feature = "aac")]
            registry.register_all::<AacDecoder>();

            #[cfg(feature = "flac")]
            registry.register_all::<FlacDecoder>();

            #[cfg(feature = "mp3")]
            registry.register_all::<Mp3Decoder>();

            #[cfg(feature = "pcm")]
            registry.register_all::<PcmDecoder>();

            registry
        };
    }

    lazy_static! {
        static ref PROBE: Probe = {
            #[cfg(feature = "aac")]
            use symphonia_codec_aac::AdtsReader;
            #[cfg(feature = "flac")]
            use symphonia_bundle_flac::FlacReader;
            #[cfg(feature = "isomp4")]
            use symphonia_format_isomp4::IsoMp4Reader;
            #[cfg(feature = "mp3")]
            use symphonia_bundle_mp3::Mp3Reader;
            #[cfg(feature = "wav")]
            use symphonia_format_wav::WavReader;
            #[cfg(feature = "ogg")]
            use symphonia_format_ogg::OggReader;

            use symphonia_metadata::id3v2::Id3v2Reader;

            let mut registry: Probe = Default::default();

            #[cfg(feature = "aac")]
            registry.register_all::<AdtsReader>();

            #[cfg(feature = "flac")]
            registry.register_all::<FlacReader>();

            #[cfg(feature = "isomp4")]
            registry.register_all::<IsoMp4Reader>();

            #[cfg(feature = "mp3")]
            registry.register_all::<Mp3Reader>();

            #[cfg(feature = "wav")]
            registry.register_all::<WavReader>();

            #[cfg(feature = "ogg")]
            registry.register_all::<OggReader>();


            registry.register_all::<Id3v2Reader>();

            registry
        };
    }

    /// Gets the default `CodecRegistry`. This registry pre-registers all the codecs selected by the
    /// `feature` flags in the includer's `Cargo.toml`. If `features` is not set, the default set of
    /// Symphonia codecs is registered.
    ///
    /// This function does not instantiate the `CodecRegistry` until the first call to this function.
    pub fn get_codecs() -> &'static CodecRegistry {
        &CODEC_REGISTRY
    }

    /// Gets the default `Probe`. This registry pre-registers all the formats selected by the
    /// `feature` flags in the includer's `Cargo.toml`. If `features` is not set, the default set of
    /// Symphonia formats is registered.
    ///
    /// This function does not instantiate the `Probe` until the first call to this function.
    pub fn get_probe() -> &'static Probe {
        &PROBE
    }

}

pub use symphonia_core as core;