// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{Result, decode_error, unsupported_error};
use symphonia_core::io::{ByteStream, FiniteStream, ScopedStream};

use crate::atoms::{Atom, AtomHeader};

use log::{debug, warn};

const ES_DESCRIPTOR: u8 = 0x03;
const DECODER_CONFIG_DESCRIPTOR: u8 = 0x04;
const DECODER_SPECIFIC_DESCRIPTOR: u8 = 0x05;
const SL_CONFIG_DESCRIPTOR: u8 = 0x06;

const MIN_DESCRIPTOR_SIZE: u64 = 2;

fn read_descriptor_header<B: ByteStream>(reader: &mut B) -> Result<(u8, u32)> {
    let tag = reader.read_u8()?;

    let mut size = 0;

    for _ in 0..4 {
        let val = reader.read_u8()?;
        size = (size << 7) | u32::from(val & 0x7f);
        if val & 0x80 == 0 {
            break;
        }
    }

    Ok((tag, size))
}

#[derive(Debug)]
pub struct EsdsAtom {
    /// Atom header.
    header: AtomHeader,
    /// Elementary stream descriptor.
    pub descriptor: ESDescriptor,
}

impl Atom for EsdsAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ByteStream>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let mut descriptor = None;

        let mut scoped = ScopedStream::new(reader, header.data_len - 4);

        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                ES_DESCRIPTOR => {
                    descriptor = Some(ESDescriptor::read(&mut scoped, desc_len)?);
                }
                _ => {
                    warn!("unknown descriptor in esds atom, desc={}", desc);
                    scoped.ignore_bytes(desc_len as u64)?;
                }
            }
        }

        // Ignore remainder of the atom.
        scoped.ignore()?;

        Ok(EsdsAtom {
            header,
            descriptor: descriptor.unwrap(),
        })

    }
}

pub trait ObjectDescriptor : Sized {
    fn read<B: ByteStream>(reader: &mut B, len: u32) -> Result<Self>;
}

/*
class ES_Descriptor extends BaseDescriptor : bit(8) tag=ES_DescrTag {
    bit(16) ES_ID;
    bit(1) streamDependenceFlag;
    bit(1) URL_Flag;
    bit(1) OCRstreamFlag;
    bit(5) streamPriority;
    if (streamDependenceFlag)
        bit(16) dependsOn_ES_ID;
    if (URL_Flag) {
        bit(8) URLlength;
        bit(8) URLstring[URLlength];
    }
    if (OCRstreamFlag)
        bit(16) OCR_ES_Id;
    DecoderConfigDescriptor decConfigDescr;
    SLConfigDescriptor slConfigDescr;
    IPI_DescrPointer ipiPtr[0 .. 1];
    IP_IdentificationDataSet ipIDS[0 .. 255];
    IPMP_DescriptorPointer ipmpDescrPtr[0 .. 255];
    LanguageDescriptor langDescr[0 .. 255];
    QoS_Descriptor qosDescr[0 .. 1];
    RegistrationDescriptor regDescr[0 .. 1];
    ExtensionDescriptor extDescr[0 .. 255];
}
*/

#[derive(Debug)]
pub struct ESDescriptor {
    pub es_id: u16,
    pub flags: u8,
    pub dec_config: DecoderConfigDescriptor,
    pub sl_config: SLDescriptor,
}

impl ObjectDescriptor for ESDescriptor {
    fn read<B: ByteStream>(reader: &mut B, len: u32) -> Result<Self> {

        let es_id = reader.read_be_u16()?;
        let flags = reader.read_u8()?;

        // All flags must be 0.
        if flags & 0xe0 != 0 {
            return unsupported_error("esdescriptor flags");
        }

        let mut dec_config = None;
        let mut sl_config = None;

        let mut scoped = ScopedStream::new(reader, u64::from(len) - 3);

        // Multiple descriptors follow, but only the decoder configuration descriptor is useful.
        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                DECODER_CONFIG_DESCRIPTOR => {
                    dec_config = Some(DecoderConfigDescriptor::read(&mut scoped, desc_len)?);
                }
                SL_CONFIG_DESCRIPTOR => {
                    sl_config = Some(SLDescriptor::read(&mut scoped, desc_len)?);
                }
                _ => {
                    debug!("skipping {} object in es descriptor", desc);
                    scoped.ignore_bytes(u64::from(desc_len))?;
                }
            }
        }

        // Consume remaining bytes.
        scoped.ignore()?;

        // Decoder configuration descriptor is mandatory.
        if dec_config.is_none() {
            return decode_error("missing decoder config descriptor");
        }

        // SL descriptor is mandatory.
        if sl_config.is_none() {
            return decode_error("missing sl config descriptor");
        }

        Ok(ESDescriptor {
            es_id,
            flags,
            dec_config: dec_config.unwrap(),
            sl_config: sl_config.unwrap(),
        })
    }
}

/*
class DecoderConfigDescriptor extends BaseDescriptor : bit(8) tag=DecoderConfigDescrTag {
    bit(8) objectTypeIndication;
    bit(6) streamType;
    bit(1) upStream;
    const bit(1) reserved=1;
    bit(24) bufferSizeDB;
    bit(32) maxBitrate;
    bit(32) avgBitrate;
    DecoderSpecificInfo decSpecificInfo[0 .. 1];
    profileLevelIndicationIndexDescriptor profileLevelIndicationIndexDescr [0..255];
}
*/
#[derive(Debug)]
pub struct DecoderConfigDescriptor {
    pub object_type_indication: u8,
    pub stream_type: u8,
    pub upstream: u8,
    pub buffer_size: u32,
    pub max_bitrate: u32,
    pub avg_bitrate: u32,
    pub dec_specific_config: DecoderSpecificInfo,
}

impl ObjectDescriptor for DecoderConfigDescriptor {
    fn read<B: ByteStream>(reader: &mut B, len: u32) -> Result<Self> {
        let object_type_indication = reader.read_u8()?;

        let (stream_type, upstream, reserved) = {
            let val = reader.read_u8()?;

            (
                (val & 0xfc) >> 2,
                (val & 0x02) >> 1,
                (val & 0x01) >> 0,
            )
        };

        if reserved != 1 {
            return decode_error("reserved bit not 1");
        }

        let buffer_size = reader.read_be_u24()?;
        let max_bitrate = reader.read_be_u32()?;
        let avg_bitrate = reader.read_be_u32()?;

        let mut dec_specific_config = None;

        let mut scoped = ScopedStream::new(reader, u64::from(len) - 13);

        // Multiple descriptors follow, but only the decoder specific info descriptor is useful.
        while scoped.bytes_available() > MIN_DESCRIPTOR_SIZE {
            let (desc, desc_len) = read_descriptor_header(&mut scoped)?;

            match desc {
                DECODER_SPECIFIC_DESCRIPTOR => {
                    dec_specific_config = Some(DecoderSpecificInfo::read(&mut scoped, desc_len)?);
                }
                _ => {
                    debug!("skipping {} object in decoder config descriptor", desc);
                    scoped.ignore_bytes(u64::from(desc_len))?;
                }
            }
        }

        // Consume remaining bytes.
        scoped.ignore()?;

        Ok(DecoderConfigDescriptor {
            object_type_indication,
            stream_type,
            upstream,
            buffer_size,
            max_bitrate,
            avg_bitrate,
            dec_specific_config: dec_specific_config.unwrap(),
        })

    }
}

#[derive(Debug)]
pub struct DecoderSpecificInfo {
    pub extra_data: Box<[u8]>,
}

impl ObjectDescriptor for DecoderSpecificInfo {
    fn read<B: ByteStream>(reader: &mut B, len: u32) -> Result<Self> {
        Ok(DecoderSpecificInfo {
            extra_data: reader.read_boxed_slice_exact(len as usize)?,
        })
    }
}

/*
class SLConfigDescriptor extends BaseDescriptor : bit(8) tag=SLConfigDescrTag {
    bit(8) predefined;
    if (predefined==0) {
        bit(1) useAccessUnitStartFlag;
        bit(1) useAccessUnitEndFlag;
        bit(1) useRandomAccessPointFlag;
        bit(1) hasRandomAccessUnitsOnlyFlag;
        bit(1) usePaddingFlag;
        bit(1) useTimeStampsFlag;
        bit(1) useIdleFlag;
        bit(1) durationFlag;
        bit(32) timeStampResolution;
        bit(32) OCRResolution;
        bit(8) timeStampLength; // must be  64
        bit(8) OCRLength; // must be  64
        bit(8) AU_Length; // must be  32
        bit(8) instantBitrateLength;
        bit(4) degradationPriorityLength;
        bit(5) AU_seqNumLength; // must be  16
        bit(5) packetSeqNumLength; // must be  16
        bit(2) reserved=0b11;
    }
    if (durationFlag) {
        bit(32) timeScale;
        bit(16) accessUnitDuration;
        bit(16) compositionUnitDuration;
    }
    if (!useTimeStampsFlag) {
        bit(timeStampLength) startDecodingTimeStamp;
        bit(timeStampLength) startCompositionTimeStamp;
    }
}

timeStampLength == 32, for predefined == 0x1
timeStampLength == 0,  for predefined == 0x2
*/
#[derive(Debug)]
pub struct SLDescriptor;

impl ObjectDescriptor for SLDescriptor {
    
    fn read<B: ByteStream>(reader: &mut B, _len: u32) -> Result<Self> {
        // const SLCONFIG_PREDEFINED_CUSTOM: u8 = 0x0;
        // const SLCONFIG_PREDEFINED_NULL: u8 = 0x1;
        const SLCONFIG_PREDEFINED_MP4: u8 = 0x2;

        let predefined = reader.read_u8()?;

        if predefined != SLCONFIG_PREDEFINED_MP4 {
            return unsupported_error("sl descriptor predefined not mp4");
        }

        Ok(SLDescriptor {})
    }
}
