// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::Result;
use symphonia_core::io::ByteStream;

use crate::atoms::{Atom, AtomHeader};

#[derive(Debug)]
pub struct StssAtom {
    /// Atom header.
    header: AtomHeader,
}

impl Atom for StssAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ByteStream>(_reader: &mut B, _header: AtomHeader) -> Result<Self> {
        todo!()
    }
}