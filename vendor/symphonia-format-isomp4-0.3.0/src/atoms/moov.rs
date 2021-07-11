// Symphonia
// Copyright (c) 2020 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{Result, decode_error};
use symphonia_core::io::ByteStream;
use symphonia_core::meta::MetadataQueue;

use crate::atoms::{Atom, AtomHeader, AtomIterator, AtomType, MvexAtom, MvhdAtom, TrakAtom, UdtaAtom};

use log::warn;

/// Movie atom.
#[derive(Debug)]
pub struct MoovAtom {
    /// Atom header.
    header: AtomHeader,
    /// Movie header atom.
    pub mvhd: MvhdAtom,
    /// Trak atoms.
    pub traks: Vec<TrakAtom>,
    /// Movie extends atom. The presence of this atom indicates a fragmented stream.
    pub mvex: Option<MvexAtom>,
    /// User data (usually metadata).
    pub udta: Option<UdtaAtom>,
}

impl MoovAtom {
    /// Consume any metadata, and pushes it onto provided `MetadataQueue`.
    pub fn take_metadata(&mut self, queue: &mut MetadataQueue) {
        if let Some(udta) = self.udta.as_mut() {
            udta.take_metadata(queue);
        }
    }

    /// Is the movie segmented.
    pub fn is_fragmented(&self) -> bool {
        self.mvex.is_some()
    }
}

impl Atom for MoovAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ByteStream>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let mut iter = AtomIterator::new(reader, header);

        let mut mvhd = None;
        let mut traks = Vec::new();
        let mut mvex = None;
        let mut udta = None;

        while let Some(header) = iter.next()? {
            match header.atype {
                AtomType::MovieHeader => {
                    mvhd = Some(iter.read_atom::<MvhdAtom>()?);
                }
                AtomType::Track => {
                    let trak = iter.read_atom::<TrakAtom>()?;
                    traks.push(trak);
                }
                AtomType::MovieExtends => {
                    mvex = Some(iter.read_atom::<MvexAtom>()?);
                }
                AtomType::UserData => {
                    udta = Some(iter.read_atom::<UdtaAtom>()?);
                }
                _ => ()
            }
        }

        if mvhd.is_none() {
            return decode_error("missing mvhd atom");
        }

        // If fragmented, the mvex atom should contain a trex atom for each trak atom in moov.
        if let Some(mvex) = mvex.as_ref() {
            // For each trak, find a matching trex atom using the track id.
            for trak in traks.iter() {
                let found = mvex.trexs.iter().find(|&trex| trex.track_id == trak.tkhd.id).is_some();

                if !found {
                    warn!("missing trex atom for trak with id={}", trak.tkhd.id);
                }
            }
        }

        Ok(MoovAtom {
            header,
            mvhd: mvhd.unwrap(),
            traks,
            mvex,
            udta,
        })
    }
}