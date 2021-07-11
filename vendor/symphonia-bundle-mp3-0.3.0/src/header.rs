// Symphonia
// Copyright (c) 2019 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{Result, decode_error, unsupported_error};
use symphonia_core::io::ByteStream;

use crate::common::*;

/// Bit-rate lookup table for MPEG version 1 layer 1.
static BIT_RATES_MPEG1_L1: [u32; 15] =
[
    0,
    32_000,  64_000,  96_000, 128_000, 160_000, 192_000, 224_000,
    256_000, 288_000, 320_000, 352_000, 384_000, 416_000, 448_000,
];

/// Bit-rate lookup table for MPEG version 1 layer 2.
static BIT_RATES_MPEG1_L2: [u32; 15] =
[
    0,
    32_000,  48_000,  56_000,  64_000,  80_000,  96_000, 112_000,
    128_000, 160_000, 192_000, 224_000, 256_000, 320_000, 384_000,
];

/// Bit-rate lookup table for MPEG version 1 layer 3.
static BIT_RATES_MPEG1_L3: [u32; 15] =
[
    0,
    32_000,  40_000,  48_000,  56_000,  64_000,  80_000,  96_000,
    112_000, 128_000, 160_000, 192_000, 224_000, 256_000, 320_000
];

/// Bit-rate lookup table for MPEG version 2 & 2.5 audio layer 1.
static BIT_RATES_MPEG2_L1: [u32; 15] =
[
    0,
    32_000,  48_000,  56_000,  64_000,  80_000,  96_000,  112_000,
    128_000, 144_000, 160_000, 176_000, 192_000, 224_000, 256_000,
];

/// Bit-rate lookup table for MPEG version 2 & 2.5 audio layers 2 & 3.
static BIT_RATES_MPEG2_L23: [u32; 15] =
[
    0,
    8_000,  16_000, 24_000, 32_000,  40_000,  48_000,  56_000,
    64_000, 80_000, 96_000, 112_000, 128_000, 144_000, 160_000,
];

/// Synchronize the provided reader to the end of the frame header, and return the frame header as
/// as `u32`.
pub fn sync_frame<B: ByteStream>(reader: &mut B) -> Result<u32> {
    let mut sync = 0u32;

    // Synchronize stream to the next frame using the sync word. The MP3 frame header always starts
    // at a byte boundary with 0xffe (11 consecutive 1 bits.) if supporting up to MPEG version 2.5.
    while (sync & 0xffe0_0000) != 0xffe0_0000 {
        sync = (sync << 8) | u32::from(reader.read_u8()?);
    }

    Ok(sync)
}

pub fn parse_frame_header(header: u32) -> Result<FrameHeader> {
    // The MP3 header is structured as follows:
    //
    // 0b1111_1111 0b111v_vlly 0brrrr_hhpx 0bmmmm_coee
    // where:
    //     vv   = version, ll = layer      , y = crc
    //     rrrr = bitrate, hh = sample rate, p = padding , x  = private bit
    //     mmmm = mode   , c  = copyright  , o = original, ee = emphasis

    let version = match (header & 0x18_0000) >> 19 {
        0b00 => MpegVersion::Mpeg2p5,
        0b10 => MpegVersion::Mpeg2,
        0b11 => MpegVersion::Mpeg1,
        _    => return decode_error("invalid MPEG version"),
    };

    let layer = match (header & 0x6_0000) >> 17 {
        0b01 => MpegLayer::Layer3,
        0b10 => MpegLayer::Layer2,
        0b11 => MpegLayer::Layer1,
        _    => return decode_error("invalid MPEG layer"),
    };

    let bitrate = match ((header & 0xf000) >> 12, version, layer) {
        // "Free" bit-rate. Note, this is NOT variable bit-rate and is not a mandatory feature of
        // MP3 decoders.
        (0b0000, _, _) => return unsupported_error("free bit-rate is not supported"),
        // Invalid bit-rate.
        (0b1111, _, _) => return decode_error("invalid bit-rate"),
        // MPEG 1 bit-rates.
        (i, MpegVersion::Mpeg1, MpegLayer::Layer1) => BIT_RATES_MPEG1_L1[i as usize],
        (i, MpegVersion::Mpeg1, MpegLayer::Layer2) => BIT_RATES_MPEG1_L2[i as usize],
        (i, MpegVersion::Mpeg1, MpegLayer::Layer3) => BIT_RATES_MPEG1_L3[i as usize],
        // MPEG 2 bit-rates.
        (i,                  _, MpegLayer::Layer1) => BIT_RATES_MPEG2_L1[i as usize],
        (i,                  _,                 _) => BIT_RATES_MPEG2_L23[i as usize],
    };

    let (sample_rate, sample_rate_idx) = match ((header & 0xc00) >> 10, version) {
        (0b00, MpegVersion::Mpeg1)   => (44_100, 0),
        (0b01, MpegVersion::Mpeg1)   => (48_000, 1),
        (0b10, MpegVersion::Mpeg1)   => (32_000, 2),
        (0b00, MpegVersion::Mpeg2)   => (22_050, 3),
        (0b01, MpegVersion::Mpeg2)   => (24_000, 4),
        (0b10, MpegVersion::Mpeg2)   => (16_000, 5),
        (0b00, MpegVersion::Mpeg2p5) => (11_025, 6),
        (0b01, MpegVersion::Mpeg2p5) => (12_000, 7),
        (0b10, MpegVersion::Mpeg2p5) => ( 8_000, 8),
        _                            => return decode_error("invalid sample rate"),
    };

    let channel_mode = match ((header & 0xc0) >> 6, layer) {
        // Stereo, for layers 1, 2, and 3.
        (0b00,                 _) => ChannelMode::Stereo,
        // Dual mono, for layers 1, 2, and 3.
        (0b10,                 _) => ChannelMode::DualMono,
        // Mono, for layers 1, 2, and 3.
        (0b11,                 _) => ChannelMode::Mono,
        // Joint stereo mode for layer 3 supports a combination of Mid-Side and Intensity Stereo
        // depending on the mode extension bits.
        (0b01, MpegLayer::Layer3) => ChannelMode::JointStereo(Mode::Layer3 {
            mid_side:  header & 0x20 != 0x0,
            intensity: header & 0x10 != 0x0,
        }),
        // Joint stereo mode for layers 1 and 2 only supports Intensity Stereo. The mode extension
        // bits indicate for which sub-bands intensity stereo coding is applied.
        (0b01,                 _) => ChannelMode::JointStereo(Mode::Intensity {
            bound: (1 + ((header & 0x30) >> 4)) << 2,
        }),
        _                         => unreachable!(),
    };

    // Some layer 2 channel and bit-rate combinations are not allowed. Check that the frame does not
    // use them.
    if layer == MpegLayer::Layer2 {
        if channel_mode == ChannelMode::Mono {
            if bitrate == 224_000
                || bitrate == 256_000
                || bitrate == 320_000
                || bitrate == 384_000
            {
                return decode_error("invalid Layer 2 bitrate for mono channel mode");
            }
        }
        else if bitrate == 32_000 || bitrate == 48_000 || bitrate == 56_000 || bitrate == 80_000 {
            return decode_error("invalid Layer 2 bitrate for non-mono channel mode");
        }
    }

    let emphasis = match header & 0x3 {
        0b00 => Emphasis::None,
        0b01 => Emphasis::Fifty15,
        0b11 => Emphasis::CcitJ17,
        _    => return decode_error("invalid emphasis"),
    };

    let is_copyrighted = header & 0x8 != 0x0;
    let is_original = header & 0x4 != 0x0;
    let has_padding = header & 0x200 != 0;

    let has_crc = header & 0x1_0000 == 0;

    // Calculate the size of the frame excluding this header.
    let frame_size =
        (if version == MpegVersion::Mpeg1 { 144 } else { 72 } * bitrate / sample_rate) as usize
        + if has_padding { 1 } else { 0 }
        - 4;

    Ok(FrameHeader{
        version,
        layer,
        bitrate,
        sample_rate,
        sample_rate_idx,
        channel_mode,
        emphasis,
        is_copyrighted,
        is_original,
        has_padding,
        has_crc,
        frame_size,
    })
}

/// Reads a MPEG audio frame header from the stream and return it or an error.
pub fn read_frame_header<B: ByteStream>(reader: &mut B) -> Result<FrameHeader> {
    // Synchronize and parse the frame header.
    parse_frame_header(sync_frame(reader)?)
}
