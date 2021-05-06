// Copyright (c) 2017 Daniel Grunwald
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this
// software and associated documentation files (the "Software"), to deal in the Software
// without restriction, including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons
// to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
// PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE
// FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! `PyBuffer` implementation
use crate::err::{self, PyResult};
use crate::{exceptions, ffi, AsPyPointer, FromPyObject, PyAny, PyNativeType, Python};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw;
use std::pin::Pin;
use std::{cell, mem, ptr, slice};

/// Allows access to the underlying buffer used by a python object such as `bytes`, `bytearray` or `array.array`.
// use Pin<Box> because Python expects that the Py_buffer struct has a stable memory address
#[repr(transparent)]
pub struct PyBuffer<T>(Pin<Box<ffi::Py_buffer>>, PhantomData<T>);

// PyBuffer is thread-safe: the shape of the buffer is immutable while a Py_buffer exists.
// Accessing the buffer contents is protected using the GIL.
unsafe impl<T> Send for PyBuffer<T> {}
unsafe impl<T> Sync for PyBuffer<T> {}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ElementType {
    SignedInteger { bytes: usize },
    UnsignedInteger { bytes: usize },
    Bool,
    Float { bytes: usize },
    Unknown,
}

impl ElementType {
    pub fn from_format(format: &CStr) -> ElementType {
        match format.to_bytes() {
            [char] | [b'@', char] => native_element_type_from_type_char(*char),
            [modifier, char]
                if (*modifier == b'='
                    || *modifier == b'<'
                    || *modifier == b'>'
                    || *modifier == b'!') =>
            {
                standard_element_type_from_type_char(*char)
            }
            _ => ElementType::Unknown,
        }
    }
}

fn native_element_type_from_type_char(type_char: u8) -> ElementType {
    use self::ElementType::*;
    match type_char {
        b'c' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_char>(),
        },
        b'b' => SignedInteger {
            bytes: mem::size_of::<raw::c_schar>(),
        },
        b'B' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_uchar>(),
        },
        b'?' => Bool,
        b'h' => SignedInteger {
            bytes: mem::size_of::<raw::c_short>(),
        },
        b'H' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_ushort>(),
        },
        b'i' => SignedInteger {
            bytes: mem::size_of::<raw::c_int>(),
        },
        b'I' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_uint>(),
        },
        b'l' => SignedInteger {
            bytes: mem::size_of::<raw::c_long>(),
        },
        b'L' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_ulong>(),
        },
        b'q' => SignedInteger {
            bytes: mem::size_of::<raw::c_longlong>(),
        },
        b'Q' => UnsignedInteger {
            bytes: mem::size_of::<raw::c_ulonglong>(),
        },
        b'n' => SignedInteger {
            bytes: mem::size_of::<libc::ssize_t>(),
        },
        b'N' => UnsignedInteger {
            bytes: mem::size_of::<libc::size_t>(),
        },
        b'e' => Float { bytes: 2 },
        b'f' => Float { bytes: 4 },
        b'd' => Float { bytes: 8 },
        _ => Unknown,
    }
}

fn standard_element_type_from_type_char(type_char: u8) -> ElementType {
    use self::ElementType::*;
    match type_char {
        b'c' | b'B' => UnsignedInteger { bytes: 1 },
        b'b' => SignedInteger { bytes: 1 },
        b'?' => Bool,
        b'h' => SignedInteger { bytes: 2 },
        b'H' => UnsignedInteger { bytes: 2 },
        b'i' | b'l' => SignedInteger { bytes: 4 },
        b'I' | b'L' => UnsignedInteger { bytes: 4 },
        b'q' => SignedInteger { bytes: 8 },
        b'Q' => UnsignedInteger { bytes: 8 },
        b'e' => Float { bytes: 2 },
        b'f' => Float { bytes: 4 },
        b'd' => Float { bytes: 8 },
        _ => Unknown,
    }
}

#[cfg(target_endian = "little")]
fn is_matching_endian(c: u8) -> bool {
    c == b'@' || c == b'=' || c == b'>'
}

#[cfg(target_endian = "big")]
fn is_matching_endian(c: u8) -> bool {
    c == b'@' || c == b'=' || c == b'>' || c == b'!'
}

/// Trait implemented for possible element types of `PyBuffer`.
pub unsafe trait Element: Copy {
    /// Gets whether the element specified in the format string is potentially compatible.
    /// Alignment and size are checked separately from this function.
    fn is_compatible_format(format: &CStr) -> bool;
}

fn validate(b: &ffi::Py_buffer) -> PyResult<()> {
    // shape and stride information must be provided when we use PyBUF_FULL_RO
    if b.shape.is_null() {
        return Err(exceptions::PyBufferError::new_err("Shape is Null"));
    }
    if b.strides.is_null() {
        return Err(exceptions::PyBufferError::new_err(
            "PyBuffer: Strides is Null",
        ));
    }
    Ok(())
}

impl<'source, T: Element> FromPyObject<'source> for PyBuffer<T> {
    fn extract(obj: &PyAny) -> PyResult<PyBuffer<T>> {
        Self::get(obj)
    }
}

impl<T: Element> PyBuffer<T> {
    /// Get the underlying buffer from the specified python object.
    pub fn get(obj: &PyAny) -> PyResult<PyBuffer<T>> {
        unsafe {
            let mut buf = Box::pin(ffi::Py_buffer::new());
            err::error_on_minusone(
                obj.py(),
                ffi::PyObject_GetBuffer(obj.as_ptr(), &mut *buf, ffi::PyBUF_FULL_RO),
            )?;
            validate(&buf)?;
            let buf = PyBuffer(buf, PhantomData);
            // Type Check
            if mem::size_of::<T>() == buf.item_size()
                && (buf.0.buf as usize) % mem::align_of::<T>() == 0
                && T::is_compatible_format(buf.format())
            {
                Ok(buf)
            } else {
                Err(exceptions::PyBufferError::new_err(
                    "Incompatible type as buffer",
                ))
            }
        }
    }

    /// Gets the pointer to the start of the buffer memory.
    ///
    /// Warning: the buffer memory might be mutated by other Python functions,
    /// and thus may only be accessed while the GIL is held.
    #[inline]
    pub fn buf_ptr(&self) -> *mut raw::c_void {
        self.0.buf
    }

    /// Gets a pointer to the specified item.
    ///
    /// If `indices.len() < self.dimensions()`, returns the start address of the sub-array at the specified dimension.
    pub fn get_ptr(&self, indices: &[usize]) -> *mut raw::c_void {
        let shape = &self.shape()[..indices.len()];
        for i in 0..indices.len() {
            assert!(indices[i] < shape[i]);
        }
        unsafe {
            ffi::PyBuffer_GetPointer(
                &*self.0 as *const ffi::Py_buffer as *mut ffi::Py_buffer,
                indices.as_ptr() as *mut usize as *mut ffi::Py_ssize_t,
            )
        }
    }

    /// Gets whether the underlying buffer is read-only.
    #[inline]
    pub fn readonly(&self) -> bool {
        self.0.readonly != 0
    }

    /// Gets the size of a single element, in bytes.
    /// Important exception: when requesting an unformatted buffer, item_size still has the value
    #[inline]
    pub fn item_size(&self) -> usize {
        self.0.itemsize as usize
    }

    /// Gets the total number of items.
    #[inline]
    pub fn item_count(&self) -> usize {
        (self.0.len as usize) / (self.0.itemsize as usize)
    }

    /// `item_size() * item_count()`.
    /// For contiguous arrays, this is the length of the underlying memory block.
    /// For non-contiguous arrays, it is the length that the logical structure would have if it were copied to a contiguous representation.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.0.len as usize
    }

    /// Gets the number of dimensions.
    ///
    /// May be 0 to indicate a single scalar value.
    #[inline]
    pub fn dimensions(&self) -> usize {
        self.0.ndim as usize
    }

    /// Returns an array of length `dimensions`. `shape()[i]` is the length of the array in dimension number `i`.
    ///
    /// May return None for single-dimensional arrays or scalar values (`dimensions() <= 1`);
    /// You can call `item_count()` to get the length of the single dimension.
    ///
    /// Despite Python using an array of signed integers, the values are guaranteed to be non-negative.
    /// However, dimensions of length 0 are possible and might need special attention.
    #[inline]
    pub fn shape(&self) -> &[usize] {
        unsafe { slice::from_raw_parts(self.0.shape as *const usize, self.0.ndim as usize) }
    }

    /// Returns an array that holds, for each dimension, the number of bytes to skip to get to the next element in the dimension.
    ///
    /// Stride values can be any integer. For regular arrays, strides are usually positive,
    /// but a consumer MUST be able to handle the case `strides[n] <= 0`.
    #[inline]
    pub fn strides(&self) -> &[isize] {
        unsafe { slice::from_raw_parts(self.0.strides, self.0.ndim as usize) }
    }

    /// An array of length ndim.
    /// If `suboffsets[n] >= 0`, the values stored along the nth dimension are pointers and the suboffset value dictates how many bytes to add to each pointer after de-referencing.
    /// A suboffset value that is negative indicates that no de-referencing should occur (striding in a contiguous memory block).
    ///
    /// If all suboffsets are negative (i.e. no de-referencing is needed), then this field must be NULL (the default value).
    #[inline]
    pub fn suboffsets(&self) -> Option<&[isize]> {
        unsafe {
            if self.0.suboffsets.is_null() {
                None
            } else {
                Some(slice::from_raw_parts(
                    self.0.suboffsets,
                    self.0.ndim as usize,
                ))
            }
        }
    }

    /// A NUL terminated string in struct module style syntax describing the contents of a single item.
    #[inline]
    pub fn format(&self) -> &CStr {
        if self.0.format.is_null() {
            CStr::from_bytes_with_nul(b"B\0").unwrap()
        } else {
            unsafe { CStr::from_ptr(self.0.format) }
        }
    }

    /// Gets whether the buffer is contiguous in C-style order (last index varies fastest when visiting items in order of memory address).
    #[inline]
    pub fn is_c_contiguous(&self) -> bool {
        unsafe {
            ffi::PyBuffer_IsContiguous(
                &*self.0 as *const ffi::Py_buffer,
                b'C' as std::os::raw::c_char,
            ) != 0
        }
    }

    /// Gets whether the buffer is contiguous in Fortran-style order (first index varies fastest when visiting items in order of memory address).
    #[inline]
    pub fn is_fortran_contiguous(&self) -> bool {
        unsafe {
            ffi::PyBuffer_IsContiguous(
                &*self.0 as *const ffi::Py_buffer,
                b'F' as std::os::raw::c_char,
            ) != 0
        }
    }

    /// Gets the buffer memory as a slice.
    ///
    /// This function succeeds if:
    /// * the buffer format is compatible with `T`
    /// * alignment and size of buffer elements is matching the expectations for type `T`
    /// * the buffer is C-style contiguous
    ///
    /// The returned slice uses type `Cell<T>` because it's theoretically possible for any call into the Python runtime
    /// to modify the values in the slice.
    pub fn as_slice<'a>(&'a self, _py: Python<'a>) -> Option<&'a [ReadOnlyCell<T>]> {
        if self.is_c_contiguous() {
            unsafe {
                Some(slice::from_raw_parts(
                    self.0.buf as *mut ReadOnlyCell<T>,
                    self.item_count(),
                ))
            }
        } else {
            None
        }
    }

    /// Gets the buffer memory as a slice.
    ///
    /// This function succeeds if:
    /// * the buffer is not read-only
    /// * the buffer format is compatible with `T`
    /// * alignment and size of buffer elements is matching the expectations for type `T`
    /// * the buffer is C-style contiguous
    ///
    /// The returned slice uses type `Cell<T>` because it's theoretically possible for any call into the Python runtime
    /// to modify the values in the slice.
    pub fn as_mut_slice<'a>(&'a self, _py: Python<'a>) -> Option<&'a [cell::Cell<T>]> {
        if !self.readonly() && self.is_c_contiguous() {
            unsafe {
                Some(slice::from_raw_parts(
                    self.0.buf as *mut cell::Cell<T>,
                    self.item_count(),
                ))
            }
        } else {
            None
        }
    }

    /// Gets the buffer memory as a slice.
    ///
    /// This function succeeds if:
    /// * the buffer format is compatible with `T`
    /// * alignment and size of buffer elements is matching the expectations for type `T`
    /// * the buffer is Fortran-style contiguous
    ///
    /// The returned slice uses type `Cell<T>` because it's theoretically possible for any call into the Python runtime
    /// to modify the values in the slice.
    pub fn as_fortran_slice<'a>(&'a self, _py: Python<'a>) -> Option<&'a [ReadOnlyCell<T>]> {
        if mem::size_of::<T>() == self.item_size() && self.is_fortran_contiguous() {
            unsafe {
                Some(slice::from_raw_parts(
                    self.0.buf as *mut ReadOnlyCell<T>,
                    self.item_count(),
                ))
            }
        } else {
            None
        }
    }

    /// Gets the buffer memory as a slice.
    ///
    /// This function succeeds if:
    /// * the buffer is not read-only
    /// * the buffer format is compatible with `T`
    /// * alignment and size of buffer elements is matching the expectations for type `T`
    /// * the buffer is Fortran-style contiguous
    ///
    /// The returned slice uses type `Cell<T>` because it's theoretically possible for any call into the Python runtime
    /// to modify the values in the slice.
    pub fn as_fortran_mut_slice<'a>(&'a self, _py: Python<'a>) -> Option<&'a [cell::Cell<T>]> {
        if !self.readonly() && self.is_fortran_contiguous() {
            unsafe {
                Some(slice::from_raw_parts(
                    self.0.buf as *mut cell::Cell<T>,
                    self.item_count(),
                ))
            }
        } else {
            None
        }
    }

    /// Copies the buffer elements to the specified slice.
    /// If the buffer is multi-dimensional, the elements are written in C-style order.
    ///
    ///  * Fails if the slice does not have the correct length (`buf.item_count()`).
    ///  * Fails if the buffer format is not compatible with type `T`.
    ///
    /// To check whether the buffer format is compatible before calling this method,
    /// you can use `<T as buffer::Element>::is_compatible_format(buf.format())`.
    /// Alternatively, `match buffer::ElementType::from_format(buf.format())`.
    pub fn copy_to_slice(&self, py: Python, target: &mut [T]) -> PyResult<()> {
        self.copy_to_slice_impl(py, target, b'C')
    }

    /// Copies the buffer elements to the specified slice.
    /// If the buffer is multi-dimensional, the elements are written in Fortran-style order.
    ///
    ///  * Fails if the slice does not have the correct length (`buf.item_count()`).
    ///  * Fails if the buffer format is not compatible with type `T`.
    ///
    /// To check whether the buffer format is compatible before calling this method,
    /// you can use `<T as buffer::Element>::is_compatible_format(buf.format())`.
    /// Alternatively, `match buffer::ElementType::from_format(buf.format())`.
    pub fn copy_to_fortran_slice(&self, py: Python, target: &mut [T]) -> PyResult<()> {
        self.copy_to_slice_impl(py, target, b'F')
    }

    fn copy_to_slice_impl(&self, py: Python, target: &mut [T], fort: u8) -> PyResult<()> {
        if mem::size_of_val(target) != self.len_bytes() {
            return Err(exceptions::PyBufferError::new_err(
                "Slice length does not match buffer length.",
            ));
        }
        unsafe {
            err::error_on_minusone(
                py,
                ffi::PyBuffer_ToContiguous(
                    target.as_ptr() as *mut raw::c_void,
                    &*self.0 as *const ffi::Py_buffer as *mut ffi::Py_buffer,
                    self.0.len,
                    fort as std::os::raw::c_char,
                ),
            )
        }
    }

    /// Copies the buffer elements to a newly allocated vector.
    /// If the buffer is multi-dimensional, the elements are written in C-style order.
    ///
    /// Fails if the buffer format is not compatible with type `T`.
    pub fn to_vec(&self, py: Python) -> PyResult<Vec<T>> {
        self.to_vec_impl(py, b'C')
    }

    /// Copies the buffer elements to a newly allocated vector.
    /// If the buffer is multi-dimensional, the elements are written in Fortran-style order.
    ///
    /// Fails if the buffer format is not compatible with type `T`.
    pub fn to_fortran_vec(&self, py: Python) -> PyResult<Vec<T>> {
        self.to_vec_impl(py, b'F')
    }

    fn to_vec_impl(&self, py: Python, fort: u8) -> PyResult<Vec<T>> {
        let item_count = self.item_count();
        let mut vec: Vec<T> = Vec::with_capacity(item_count);
        unsafe {
            // Copy the buffer into the uninitialized space in the vector.
            // Due to T:Copy, we don't need to be concerned with Drop impls.
            err::error_on_minusone(
                py,
                ffi::PyBuffer_ToContiguous(
                    vec.as_mut_ptr() as *mut raw::c_void,
                    &*self.0 as *const ffi::Py_buffer as *mut ffi::Py_buffer,
                    self.0.len,
                    fort as std::os::raw::c_char,
                ),
            )?;
            // set vector length to mark the now-initialized space as usable
            vec.set_len(item_count);
        }
        Ok(vec)
    }

    /// Copies the specified slice into the buffer.
    /// If the buffer is multi-dimensional, the elements in the slice are expected to be in C-style order.
    ///
    ///  * Fails if the buffer is read-only.
    ///  * Fails if the slice does not have the correct length (`buf.item_count()`).
    ///  * Fails if the buffer format is not compatible with type `T`.
    ///
    /// To check whether the buffer format is compatible before calling this method,
    /// use `<T as buffer::Element>::is_compatible_format(buf.format())`.
    /// Alternatively, `match buffer::ElementType::from_format(buf.format())`.
    pub fn copy_from_slice(&self, py: Python, source: &[T]) -> PyResult<()> {
        self.copy_from_slice_impl(py, source, b'C')
    }

    /// Copies the specified slice into the buffer.
    /// If the buffer is multi-dimensional, the elements in the slice are expected to be in Fortran-style order.
    ///
    ///  * Fails if the buffer is read-only.
    ///  * Fails if the slice does not have the correct length (`buf.item_count()`).
    ///  * Fails if the buffer format is not compatible with type `T`.
    ///
    /// To check whether the buffer format is compatible before calling this method,
    /// use `<T as buffer::Element>::is_compatible_format(buf.format())`.
    /// Alternatively, `match buffer::ElementType::from_format(buf.format())`.
    pub fn copy_from_fortran_slice(&self, py: Python, source: &[T]) -> PyResult<()> {
        self.copy_from_slice_impl(py, source, b'F')
    }

    fn copy_from_slice_impl(&self, py: Python, source: &[T], fort: u8) -> PyResult<()> {
        if self.readonly() {
            return buffer_readonly_error();
        }
        if mem::size_of_val(source) != self.len_bytes() {
            return Err(exceptions::PyBufferError::new_err(
                "Slice length does not match buffer length.",
            ));
        }
        unsafe {
            err::error_on_minusone(
                py,
                ffi::PyBuffer_FromContiguous(
                    &*self.0 as *const ffi::Py_buffer as *mut ffi::Py_buffer,
                    source.as_ptr() as *mut raw::c_void,
                    self.0.len,
                    fort as std::os::raw::c_char,
                ),
            )
        }
    }

    pub fn release(self, _py: Python) {
        // First move self into a ManuallyDrop, so that PyBuffer::drop will
        // never be called. (It would acquire the GIL and call PyBuffer_Release
        // again.)
        let mut mdself = mem::ManuallyDrop::new(self);
        unsafe {
            // Next, make the actual PyBuffer_Release call.
            ffi::PyBuffer_Release(&mut *mdself.0);

            // Finally, drop the contained Pin<Box<_>> in place, to free the
            // Box memory.
            let inner: *mut Pin<Box<ffi::Py_buffer>> = &mut mdself.0;
            ptr::drop_in_place(inner);
        }
    }
}

#[inline(always)]
fn buffer_readonly_error() -> PyResult<()> {
    Err(exceptions::PyBufferError::new_err(
        "Cannot write to read-only buffer.",
    ))
}

impl<T> Drop for PyBuffer<T> {
    fn drop(&mut self) {
        Python::with_gil(|_| unsafe { ffi::PyBuffer_Release(&mut *self.0) });
    }
}

/// Like `std::mem::cell`, but only provides read-only access to the data.
///
/// `&ReadOnlyCell<T>` is basically a safe version of `*const T`:
///  The data cannot be modified through the reference, but other references may
///  be modifying the data.
#[repr(transparent)]
pub struct ReadOnlyCell<T: Element>(cell::UnsafeCell<T>);

impl<T: Element> ReadOnlyCell<T> {
    #[inline]
    pub fn get(&self) -> T {
        unsafe { *self.0.get() }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.0.get()
    }
}

macro_rules! impl_element(
    ($t:ty, $f:ident) => {
        unsafe impl Element for $t {
            fn is_compatible_format(format: &CStr) -> bool {
                let slice = format.to_bytes();
                if slice.len() > 1 && !is_matching_endian(slice[0]) {
                    return false;
                }
                ElementType::from_format(format) == ElementType::$f { bytes: mem::size_of::<$t>() }
            }
        }
    }
);

impl_element!(u8, UnsignedInteger);
impl_element!(u16, UnsignedInteger);
impl_element!(u32, UnsignedInteger);
impl_element!(u64, UnsignedInteger);
impl_element!(usize, UnsignedInteger);
impl_element!(i8, SignedInteger);
impl_element!(i16, SignedInteger);
impl_element!(i32, SignedInteger);
impl_element!(i64, SignedInteger);
impl_element!(isize, SignedInteger);
impl_element!(f32, Float);
impl_element!(f64, Float);

#[cfg(test)]
mod test {
    use super::PyBuffer;
    use crate::ffi;
    use crate::Python;

    #[test]
    fn test_compatible_size() {
        // for the cast in PyBuffer::shape()
        assert_eq!(
            std::mem::size_of::<ffi::Py_ssize_t>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_bytes_buffer() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let bytes = py.eval("b'abcde'", None, None).unwrap();
        let buffer = PyBuffer::get(&bytes).unwrap();
        assert_eq!(buffer.dimensions(), 1);
        assert_eq!(buffer.item_count(), 5);
        assert_eq!(buffer.format().to_str().unwrap(), "B");
        assert_eq!(buffer.shape(), [5]);
        // single-dimensional buffer is always contiguous
        assert!(buffer.is_c_contiguous());
        assert!(buffer.is_fortran_contiguous());

        let slice = buffer.as_slice(py).unwrap();
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[0].get(), b'a');
        assert_eq!(slice[2].get(), b'c');

        assert!(buffer.copy_to_slice(py, &mut [0u8]).is_err());
        let mut arr = [0; 5];
        buffer.copy_to_slice(py, &mut arr).unwrap();
        assert_eq!(arr, b"abcde" as &[u8]);

        assert!(buffer.copy_from_slice(py, &[0u8; 5]).is_err());
        assert_eq!(buffer.to_vec(py).unwrap(), b"abcde");
    }

    #[allow(clippy::float_cmp)] // The test wants to ensure that no precision was lost on the Python round-trip
    #[test]
    fn test_array_buffer() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let array = py
            .import("array")
            .unwrap()
            .call_method("array", ("f", (1.0, 1.5, 2.0, 2.5)), None)
            .unwrap();
        let buffer = PyBuffer::get(array).unwrap();
        assert_eq!(buffer.dimensions(), 1);
        assert_eq!(buffer.item_count(), 4);
        assert_eq!(buffer.format().to_str().unwrap(), "f");
        assert_eq!(buffer.shape(), [4]);

        let slice = buffer.as_slice(py).unwrap();
        assert_eq!(slice.len(), 4);
        assert_eq!(slice[0].get(), 1.0);
        assert_eq!(slice[3].get(), 2.5);

        let mut_slice = buffer.as_mut_slice(py).unwrap();
        assert_eq!(mut_slice.len(), 4);
        assert_eq!(mut_slice[0].get(), 1.0);
        mut_slice[3].set(2.75);
        assert_eq!(slice[3].get(), 2.75);

        buffer
            .copy_from_slice(py, &[10.0f32, 11.0, 12.0, 13.0])
            .unwrap();
        assert_eq!(slice[2].get(), 12.0);

        assert_eq!(buffer.to_vec(py).unwrap(), [10.0, 11.0, 12.0, 13.0]);
    }
}
