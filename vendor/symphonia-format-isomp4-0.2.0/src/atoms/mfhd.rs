// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ByteStream;

use crate::atoms::{Atom, AtomHeader};

/// Movie fragment header atom.
#[derive(Debug)]
pub struct MfhdAtom {
    /// Atom header.
    header: AtomHeader,
    /// Sequence number associated with fragment.
    pub sequence_number: u32,
}

impl Atom for MfhdAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ByteStream>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, _) = AtomHeader::read_extra(reader)?;

        let sequence_number = reader.read_be_u32()?;

        Ok(MfhdAtom {
            header,
            sequence_number,
        })
    }
}