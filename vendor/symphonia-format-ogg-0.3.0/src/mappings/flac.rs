// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::OggPacket;

use super::{Bitstream, Mapper, MapResult};

use symphonia_core::checksum::Crc8Ccitt;
use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_FLAC};
use symphonia_core::errors::{Result, decode_error};
use symphonia_core::meta::MetadataBuilder;
use symphonia_core::io::{BufStream, ByteStream, MonitorStream};

use symphonia_utils_xiph::flac::metadata::{MetadataBlockHeader, MetadataBlockType, StreamInfo};
use symphonia_utils_xiph::flac::metadata::{read_comment_block, read_picture_block};

/// The expected size of the first FLAC header packet.
const OGG_FLAC_HEADER_PACKET_SIZE: usize = 51;

/// The major version number of the supported OGG-FLAC mapping.
const OGG_FLAC_MAPPING_MAJOR_VERSION: u8 = 1;

/// The OGG-FLAC header packet signature.
const OGG_FLAC_HEADER_SIGNATURE: &[u8] = b"FLAC";

/// The OGG-FLAC header packet type value.
const OGG_FLAC_PACKET_TYPE: u8 = 0x7f;

/// The native FLAC signature.
const FLAC_SIGNATURE: &[u8] = b"fLaC";

pub fn detect(buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    // The packet shall be exactly the expected length.
    if buf.len() != OGG_FLAC_HEADER_PACKET_SIZE {
        return Ok(None);
    }

    let mut reader = BufStream::new(&buf);

    // The first byte indicates the packet type and must be 0x7f.
    if reader.read_u8()? != OGG_FLAC_PACKET_TYPE {
        return Ok(None);
    }

    // Next, the OGG FLAC signature, in ASCII, must be "FLAC".
    if reader.read_quad_bytes()? != OGG_FLAC_HEADER_SIGNATURE {
        return Ok(None);
    }

    // Next, a one-byte binary major version number for the mapping, only version 1 is supported.
    if reader.read_u8()? != OGG_FLAC_MAPPING_MAJOR_VERSION {
        return Ok(None);
    }

    // Next, a one-byte minor version number for the mapping. This is ignored because we support all
    // version 1 features.
    let _minor = reader.read_u8()?;

    // Next, a two-byte, big-endian number signifying the number of header (non-audio) packets, not
    // including the identification packet. This number may be 0 to signify it is unknown.
    let _ = reader.read_be_u16()?;

    // Last, the four-byte ASCII native FLAC signature "fLaC".
    if reader.read_quad_bytes()? != FLAC_SIGNATURE {
        return Ok(None);
    }

    // Following the previous OGG FLAC identification data is the stream information block as a
    // native FLAC metadata block.
    let header = MetadataBlockHeader::read(&mut reader)?;

    if header.block_type != MetadataBlockType::StreamInfo {
        return Ok(None);
    }

    let stream_info = StreamInfo::read(&mut reader)?;

    // Populate the codec parameters with the information read from the stream information block.
    let mut codec_params = CodecParameters::new();

    codec_params
        .for_codec(CODEC_TYPE_FLAC)
        .with_sample_rate(stream_info.sample_rate)
        .with_bits_per_sample(stream_info.bits_per_sample)
        .with_max_frames_per_packet(u64::from(stream_info.block_len_max))
        .with_channels(stream_info.channels)
        .with_packet_data_integrity(true);

    if let Some(n_frames) = stream_info.n_samples {
        codec_params.with_n_frames(n_frames);
    }

    // Instantiate the FLAC mapper.
    let mapper = Box::new(FlacMapper {
        codec_params,
    });

    Ok(Some(mapper))
}

/// Decodes a big-endian unsigned integer encoded via extended UTF8.
fn utf8_decode_be_u64<B: ByteStream>(src : &mut B) -> Result<Option<u64>> {
    // NOTE: See the symphonia-bundle-flac crate for a detailed description of this function.
    let mut state = u64::from(src.read_u8()?);

    let mask: u8 = match state {
        0x00..=0x7f => return Ok(Some(state)),
        0xc0..=0xdf => 0x1f,
        0xe0..=0xef => 0x0f,
        0xf0..=0xf7 => 0x07,
        0xf8..=0xfb => 0x03,
        0xfc..=0xfd => 0x01,
        0xfe        => 0x00,
        _           => return Ok(None)
    };

    state &= u64::from(mask);

    for _ in 2..mask.leading_zeros() {
        state = (state << 6) | u64::from(src.read_u8()? & 0x3f);
    }

    Ok(Some(state))
}

struct FrameHeader {
    ts: u64,
    dur: u64,
}

/// Try to decode a FLAC frame header from the provided buffer.
fn decode_frame_header(buf: &[u8]) -> Result<FrameHeader> {
    // The FLAC frame header is checksummed with a CRC-8 hash.
    let mut reader_crc8 = MonitorStream::new(BufStream::new(buf), Crc8Ccitt::new(0));

    // Read the sync word.
    let sync = reader_crc8.read_be_u16()?;

    // Within an OGG packet the frame should be synchronized.
    if sync & 0xfffc != 0xfff8 {
        return decode_error("ogg: flac header is not synchronized");
    }

    // Read all the standard frame description fields as one 16-bit value and extract the fields.
    let desc = reader_crc8.read_be_u16()?;

    // Reserved bit field.
    if desc & 0x0001 == 1 {
        return decode_error("ogg: flac frame header reserved bit is not set to 1");
    }

    // Extract the blocking strategy from the sync word.
    let is_fixed_block_size = sync & 0x1 == 0;

    let block_sequence = if is_fixed_block_size {
        // Fixed block size stream sequence blocks by a frame number.
        let frame = match utf8_decode_be_u64(&mut reader_crc8)? {
            Some(frame) => frame,
            None => return decode_error("ogg: flac frame sequence number is not valid"),
        };

        // The frame number should only be 31-bits.
        if frame > 0x7fff_ffff {
            return decode_error("ogg: flac frame sequence number exceeds 31-bits");
        }

        frame
    }
    else {
        // Variable block size streams sequence blocks by a sample number.
        let sample = match utf8_decode_be_u64(&mut reader_crc8)? {
            Some(sample) => sample,
            None => return decode_error("ogg: sample sequence number is not valid"),
        };

        // The sample number should only be 36-bits.
        if sample > 0xffff_fffff {
            return decode_error("ogg: sample sequence number exceeds 36-bits");
        }

        sample
    };

    // The block size provides the duration in samples.
    let block_size_enc = u32::from((desc & 0xf000) >> 12);

    let block_size = match block_size_enc {
        0x1       => 192,
        0x2..=0x5 => 576 * (1 << (block_size_enc - 2)),
        0x6       => u64::from(reader_crc8.read_u8()?) + 1,
        0x7       => {
            let block_size = reader_crc8.read_be_u16()?;
            if block_size == 0xffff {
                return decode_error("ogg: flac block size not allowed to be greater than 65535");
            }

            u64::from(block_size) + 1
        },
        0x8..=0xf => 256 * (1 << (block_size_enc - 8)),
        _         => return decode_error("ogg: flac block size set to reserved value"),
    };

    // The sample rate is not required but should be read so checksum verification of the header
    // can be performed.
    let sample_rate_enc = u32::from((desc & 0x0f00) >>  8);

    match sample_rate_enc {
        0xc => {
            reader_crc8.read_u8()?;
        }
        0xd => {
            reader_crc8.read_be_u16()?;
        }
        0xe => {
            reader_crc8.read_be_u16()?;
        }
        _ => (),
    }

    // End of frame header, get the computed CRC-8 checksum.
    let crc8_computed = reader_crc8.monitor().crc();

    // Read the expected CRC-8 checksum from the frame header.
    let crc8_expected = reader_crc8.into_inner().read_u8()?;

    if crc8_expected != crc8_computed {
        return decode_error("ogg: flac computed frame header CRC does not match expected CRC");
    }

    let ts = if is_fixed_block_size { block_sequence * block_size } else { block_sequence };

    Ok(FrameHeader { ts, dur: block_size })
}

struct FlacMapper {
    codec_params: CodecParameters,
}

impl Mapper for FlacMapper {

    fn codec(&self) -> &CodecParameters {
        &self.codec_params
    }

    fn map_packet(&mut self, packet: &OggPacket) -> Result<MapResult> {
        // The first byte of a packet is the packet type.
        if packet.data[0] == 0xff {
            // A packet-type of 0xff is a bitstream packet.

            // The FLAC OGG encapsulation wraps an entire FLAC frame including the header into an OGG
            // packet. This duplicates some information however it means that by decoding the FLAC
            // frame header the timestamp and duration are readily accessible.
            let frame_header = decode_frame_header(&packet.data)?;

            Ok(MapResult::Bitstream(Bitstream { ts: frame_header.ts, dur: frame_header.dur }))
        }
        else if packet.data[0] == 0x00 || packet.data[0] == 0x80 {
            // Packet types 0x00 and 0x80 are invalid in the OGG mapping.
            decode_error("invalid packet type")
        }
        else {
            // Packet types in the range 0x01 thru 0x7f, and 0x81 thru 0xfe are metadata blocks.
            let mut reader = BufStream::new(&packet.data);
            let header = MetadataBlockHeader::read(&mut reader)?;

            match header.block_type {
                MetadataBlockType::VorbisComment => {
                    let mut builder = MetadataBuilder::new();

                    read_comment_block(&mut reader, &mut builder)?;

                    Ok(MapResult::Metadata(builder.metadata()))
                }
                MetadataBlockType::Picture => {
                    let mut builder = MetadataBuilder::new();

                    read_picture_block(&mut reader, &mut builder)?;

                    Ok(MapResult::Metadata(builder.metadata()))
                }
                _ => Ok(MapResult::Unknown)
            }
        }
    }

}
