// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// Previous Author: Kostya Shishkov <kostya.shiskov@gmail.com>
//
// This source file includes code originally written for the NihAV
// project. With the author's permission, it has been relicensed for,
// and ported to the Symphonia project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::Channels;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4AType {
    None,
    Main,
    LC,
    SSR,
    LTP,
    SBR,
    Scalable,
    TwinVQ,
    CELP,
    HVXC,
    TTSI,
    MainSynth,
    WavetableSynth,
    GeneralMIDI,
    Algorithmic,
    ER_AAC_LC,
    ER_AAC_LTP,
    ER_AAC_Scalable,
    ER_TwinVQ,
    ER_BSAC,
    ER_AAC_LD,
    ER_CELP,
    ER_HVXC,
    ER_HILN,
    ER_Parametric,
    SSC,
    PS,
    MPEGSurround,
    Layer1,
    Layer2,
    Layer3,
    DST,
    ALS,
    SLS,
    SLSNonCore,
    ER_AAC_ELD,
    SMRSimple,
    SMRMain,
    Reserved,
    Unknown,
}

pub const M4A_TYPES: &[M4AType] = &[
    M4AType::None,
    M4AType::Main,
    M4AType::LC,
    M4AType::SSR,
    M4AType::LTP,
    M4AType::SBR,
    M4AType::Scalable,
    M4AType::TwinVQ,
    M4AType::CELP,
    M4AType::HVXC,
    M4AType::Reserved,
    M4AType::Reserved,
    M4AType::TTSI,
    M4AType::MainSynth,
    M4AType::WavetableSynth,
    M4AType::GeneralMIDI,
    M4AType::Algorithmic,
    M4AType::ER_AAC_LC,
    M4AType::Reserved,
    M4AType::ER_AAC_LTP,
    M4AType::ER_AAC_Scalable,
    M4AType::ER_TwinVQ,
    M4AType::ER_BSAC,
    M4AType::ER_AAC_LD,
    M4AType::ER_CELP,
    M4AType::ER_HVXC,
    M4AType::ER_HILN,
    M4AType::ER_Parametric,
    M4AType::SSC,
    M4AType::PS,
    M4AType::MPEGSurround,
    M4AType::Reserved, /*escape*/
    M4AType::Layer1,
    M4AType::Layer2,
    M4AType::Layer3,
    M4AType::DST,
    M4AType::ALS,
    M4AType::SLS,
    M4AType::SLSNonCore,
    M4AType::ER_AAC_ELD,
    M4AType::SMRSimple,
    M4AType::SMRMain,
];

pub const M4A_TYPE_NAMES: &[&str] = &[
    "None",
    "AAC Main",
    "AAC LC",
    "AAC SSR",
    "AAC LTP",
    "SBR",
    "AAC Scalable",
    "TwinVQ",
    "CELP",
    "HVXC",
    // "(reserved10)",
    // "(reserved11)",
    "TTSI",
    "Main synthetic",
    "Wavetable synthesis",
    "General MIDI",
    "Algorithmic Synthesis and Audio FX",
    "ER AAC LC",
    // "(reserved18)",
    "ER AAC LTP",
    "ER AAC Scalable",
    "ER TwinVQ",
    "ER BSAC",
    "ER AAC LD",
    "ER CELP",
    "ER HVXC",
    "ER HILN",
    "ER Parametric",
    "SSC",
    "PS",
    "MPEG Surround",
    // "(escape)",
    "Layer-1",
    "Layer-2",
    "Layer-3",
    "DST",
    "ALS",
    "SLS",
    "SLS non-core",
    "ER AAC ELD",
    "SMR Simple",
    "SMR Main",
    "(reserved)",
    "(unknown)",
];

pub const AAC_SAMPLE_RATES: [u32; 16] = [
    96000, 88200, 64000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 8000, 7350, 0, 0,
    0,
];

pub const AAC_CHANNELS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 8];

pub fn map_channels(channels: u32) -> Option<Channels> {
    match channels {
        0 => None,
        1 => Some(Channels::FRONT_LEFT),
        2 => Some(Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        3 => Some(Channels::FRONT_CENTRE | Channels::FRONT_LEFT | Channels::FRONT_RIGHT),
        4 => {
            Some(
                Channels::FRONT_CENTRE
                | Channels::FRONT_LEFT
                | Channels::FRONT_RIGHT
                | Channels::REAR_CENTRE
            )
        }
        5 => {
            Some(
                Channels::FRONT_CENTRE
                | Channels::FRONT_LEFT
                | Channels::FRONT_RIGHT
                | Channels::SIDE_LEFT
                | Channels::SIDE_RIGHT
            )
        }
        6 => {
            Some(
                Channels::FRONT_CENTRE
                | Channels::FRONT_LEFT
                | Channels::FRONT_RIGHT
                | Channels::SIDE_LEFT
                | Channels::SIDE_RIGHT
                | Channels::LFE1
            )
        }
        7 => None,
        8 => {
            Some(
                Channels::FRONT_CENTRE
                | Channels::FRONT_LEFT
                | Channels::FRONT_RIGHT
                | Channels::SIDE_LEFT
                | Channels::SIDE_RIGHT
                | Channels::FRONT_LEFT_WIDE
                | Channels::FRONT_RIGHT_WIDE
                | Channels::LFE1
            )
        }
        _ => None
    }

}