// Symphonia
// Copyright (c) 2019 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `io` module implements composable stream-based I/O.

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io;
use std::mem;

mod bit;
mod buf_stream;
mod media_source_stream;
mod monitor_stream;
mod scoped_stream;

pub use bit::*;
pub use buf_stream::BufStream;
pub use media_source_stream::{MediaSourceStream, MediaSourceStreamOptions};
pub use monitor_stream::{Monitor, MonitorStream};
pub use scoped_stream::ScopedStream;

/// A `MediaSource` is a composite trait of `std::io::Read` and `std::io::Seek`. Despite requiring
/// the `Seek` trait, seeking is an optional capability that can be queried at runtime.
pub trait MediaSource: io::Read + io::Seek + Send {
    /// Returns if the source is seekable. This may be an expensive operation.
    fn is_seekable(&self) -> bool;

    /// Returns the length in bytes, if available. This may be an expensive operation.
    fn len(&self) -> Option<u64>;
}

impl MediaSource for std::fs::File {
    /// Returns if the `std::io::File` backing the `MediaSource` is seekable.
    ///
    /// Note: This operation involves querying the underlying file descriptor for information and
    /// may be moderately expensive. Therefore it is recommended to cache this value if used often.
    fn is_seekable(&self) -> bool {
        // If the file's metadata is available, and the file is a regular file (i.e., not a FIFO,
        // etc.), then the MediaSource will be seekable. Otherwise assume it is not. Note that
        // metadata() follows symlinks.
        match self.metadata() {
            Ok(metadata) => metadata.is_file(),
            _ => false
        }
    }

    /// Returns the length in bytes of the `std::io::File` backing the `MediaSource`.
    ///
    /// Note: This operation involves querying the underlying file descriptor for information and
    /// may be moderately expensive. Therefore it is recommended to cache this value if used often.
    fn len(&self) -> Option<u64> {
        match self.metadata() {
            Ok(metadata) => Some(metadata.len()),
            _ => None,
        }
    }
}

impl<T: std::convert::AsRef<[u8]> + Send> MediaSource for io::Cursor<T> {
    /// Always returns true since a `io::Cursor<u8>` is always seekable.
    fn is_seekable(&self) -> bool {
        true
    }

    /// Returns the length in bytes of the `io::Cursor<u8>` backing the `MediaSource`.
    fn len(&self) -> Option<u64> {
        // Get the underlying container, usually &Vec<T>.
        let inner = self.get_ref();
        // Get slice from the underlying container, &[T], for the len() function.
        Some(inner.as_ref().len() as u64)
    }
}

/// `ReadOnlySource` implements an unseekable `MediaSource` for any reader that implements the
/// `std::io::Read` trait.
pub struct ReadOnlySource<R: io::Read> {
    inner: R,
}

impl<R: io::Read + Send> ReadOnlySource<R> {
    /// Instantiates a new `ReadOnlySource<R>` by taking ownership and wrapping the provided
    /// `Read`er.
    pub fn new(inner: R) -> Self {
        ReadOnlySource { inner }
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Unwraps this `ReadOnlySource<R>`, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: io::Read + Send> MediaSource for ReadOnlySource<R> {
    fn is_seekable(&self) -> bool {
        false
    }

    fn len(&self) -> Option<u64> {
        None
    }
}

impl<R: io::Read> io::Read for ReadOnlySource<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::Read> io::Seek for ReadOnlySource<R> {
    fn seek(&mut self, _: io::SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(io::ErrorKind::Other, "source does not support seeking"))
    }
}

/// A `ByteStream` provides functions to read bytes and interpret them as little- or big-endian
/// unsigned integers or floating-point values of standard widths.
pub trait ByteStream {

    /// Reads a single byte from the stream and returns it or an error.
    fn read_byte(&mut self) -> io::Result<u8>;

    /// Reads two bytes from the stream and returns them in read-order or an error.
    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]>;

    /// Reads three bytes from the stream and returns them in read-order or an error.
    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]>;

    /// Reads four bytes from the stream and returns them in read-order or an error.
    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]>;

    /// Reads up-to the number of bytes required to fill buf or returns an error.
    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize>;

    /// Reads exactly the number of bytes required to fill be provided buffer or returns an error.
    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;

    /// Reads a single unsigned byte from the stream and returns it or an error.
    #[inline(always)]
    fn read_u8(&mut self) -> io::Result<u8> {
        self.read_byte()
    }

    /// Reads two bytes from the stream and interprets them as an unsigned 16-bit little-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_u16(&mut self) -> io::Result<u16> {
        Ok(u16::from_le_bytes(self.read_double_bytes()?))
    }

    /// Reads two bytes from the stream and interprets them as an unsigned 16-bit big-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_be_u16(&mut self) -> io::Result<u16> {
        Ok(u16::from_be_bytes(self.read_double_bytes()?))
    }

    /// Reads three bytes from the stream and interprets them as an unsigned 24-bit little-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_u24(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; mem::size_of::<u32>()];
        buf[0..3].clone_from_slice(&self.read_triple_bytes()?);
        Ok(u32::from_le_bytes(buf))
    }

    /// Reads three bytes from the stream and interprets them as an unsigned 24-bit big-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_be_u24(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; mem::size_of::<u32>()];
        buf[0..3].clone_from_slice(&self.read_triple_bytes()?);
        Ok(u32::from_be_bytes(buf) >> 8)
    }

    /// Reads four bytes from the stream and interprets them as an unsigned 32-bit little-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_u32(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.read_quad_bytes()?))
    }

    /// Reads four bytes from the stream and interprets them as an unsigned 32-bit big-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_be_u32(&mut self) -> io::Result<u32> {
        Ok(u32::from_be_bytes(self.read_quad_bytes()?))
    }

    /// Reads eight bytes from the stream and interprets them as an unsigned 64-bit little-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; mem::size_of::<u64>()];
        self.read_buf_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Reads eight bytes from the stream and interprets them as an unsigned 64-bit big-endian
    /// integer or returns an error.
    #[inline(always)]
    fn read_be_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; mem::size_of::<u64>()];
        self.read_buf_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    /// Reads four bytes from the stream and interprets them as a 32-bit little-endiann IEEE-754
    /// floating-point value.
    #[inline(always)]
    fn read_f32(&mut self) -> io::Result<f32> {
        Ok(LittleEndian::read_f32(&self.read_quad_bytes()?))
    }

    /// Reads four bytes from the stream and interprets them as a 32-bit big-endiann IEEE-754
    /// floating-point value.
    #[inline(always)]
    fn read_be_f32(&mut self) -> io::Result<f32> {
        Ok(BigEndian::read_f32(&self.read_quad_bytes()?))
    }

    /// Reads four bytes from the stream and interprets them as a 64-bit little-endiann IEEE-754
    /// floating-point value.
    #[inline(always)]
    fn read_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; mem::size_of::<u64>()];
        self.read_buf_exact(&mut buf)?;
        Ok(LittleEndian::read_f64(&buf))
    }

    /// Reads four bytes from the stream and interprets them as a 64-bit big-endiann IEEE-754
    /// floating-point value.
    #[inline(always)]
    fn read_be_f64(&mut self) -> io::Result<f64> {
        let mut buf = [0u8; mem::size_of::<u64>()];
        self.read_buf_exact(&mut buf)?;
        Ok(BigEndian::read_f64(&buf))
    }

    /// Reads up-to the number of bytes requested, and returns a boxed slice of the data or an
    /// error.
    fn read_boxed_slice(&mut self, len: usize) -> io::Result<Box<[u8]>> {
        let mut buf = vec![0u8; len];
        let actual_len = self.read_buf(&mut buf)?;
        buf.truncate(actual_len);
        Ok(buf.into_boxed_slice())
    }

    /// Reads exactly the number of bytes requested, and returns a boxed slice of the data or an
    /// error.
    fn read_boxed_slice_exact(&mut self, len: usize) -> io::Result<Box<[u8]>> {
        let mut buf = vec![0u8; len];
        self.read_buf_exact(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }

    /// Reads bytes from the stream into a supplied buffer until a byte pattern is matched. Returns
    /// a mutable slice to the valid region of the provided buffer.
    #[inline(always)]
    fn scan_bytes<'a>(&mut self, pattern: &[u8], buf: &'a mut [u8]) -> io::Result<&'a mut [u8]> {
        self.scan_bytes_aligned(pattern, 1, buf)
    }

    /// Reads bytes from a stream into a supplied buffer until a byte patter is matched on an
    /// aligned byte boundary. Returns a mutable slice to the valid region of the provided buffer.
    fn scan_bytes_aligned<'a>(
        &mut self, pattern: &[u8],
        align: usize,
        buf: &'a mut [u8]
    ) -> io::Result<&'a mut [u8]>;

    /// Ignores the specified number of bytes from the stream or returns an error.
    fn ignore_bytes(&mut self, count: u64) -> io::Result<()>;

    /// Gets the position of the stream.
    fn pos(&self) -> u64;
}

impl<'b, B: ByteStream> ByteStream for &'b mut B {
    #[inline(always)]
    fn read_byte(&mut self) -> io::Result<u8> {
        (*self).read_byte()
    }

    #[inline(always)]
    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        (*self).read_double_bytes()
    }

    #[inline(always)]
    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        (*self).read_triple_bytes()
    }

    #[inline(always)]
    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        (*self).read_quad_bytes()
    }

    #[inline(always)]
    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (*self).read_buf(buf)
    }

    #[inline(always)]
    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (*self).read_buf_exact(buf)
    }

    #[inline(always)]
    fn scan_bytes_aligned<'a>(
        &mut self,
        pattern: &[u8],
        align: usize,
        buf: &'a mut [u8]
    ) -> io::Result<&'a mut [u8]> {
        (*self).scan_bytes_aligned(pattern, align, buf)
    }

    #[inline(always)]
    fn ignore_bytes(&mut self, count: u64) -> io::Result<()> {
        (*self).ignore_bytes(count)
    }

    #[inline(always)]
    fn pos(&self) -> u64 {
        (**self).pos()
    }
}

/// A `FiniteStream` is a stream that has a definitive length. A `FiniteStream` therefore knows how
/// many bytes are available for reading, or have been previously read.
pub trait FiniteStream {
    /// Returns the length of the the stream.
    fn len(&self) -> u64;

    /// Returns the number of bytes read.
    fn bytes_read(&self) -> u64;

    /// Returns the number of bytes available for reading.
    fn bytes_available(&self) -> u64;
}