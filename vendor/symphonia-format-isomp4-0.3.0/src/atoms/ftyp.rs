// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{Result, decode_error};
use symphonia_core::io::ByteStream;

use crate::atoms::{Atom, AtomHeader};
use crate::fourcc::FourCc;

/// File type atom.
#[derive(Debug)]
pub struct FtypAtom {
    header: AtomHeader,
    pub major: FourCc,
    pub minor: [u8; 4],
    pub compatible: Vec<FourCc>,
}

impl Atom for FtypAtom {
    fn read<B: ByteStream>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        // The Ftyp atom must be a multiple of 4 since it only stores FourCCs.
        if header.data_len & 0x3 != 0 {
            return decode_error("invalid ftyp data length");
        }
    
        // Major
        let major = FourCc::new(reader.read_quad_bytes()?);

        // Minor
        let minor = reader.read_quad_bytes()?;

        // The remainder of the Ftyp atom contains the FourCCs of compatible brands.
        let n_brands = (header.data_len - 8) / 4;

        let mut compatible = Vec::new();

        for _ in 0..n_brands {
            let brand = reader.read_quad_bytes()?;
            compatible.push(FourCc::new(brand));
        }

        Ok(FtypAtom { header, major, minor, compatible })
    }

    fn header(&self) -> AtomHeader {
        self.header
    }
    
}
