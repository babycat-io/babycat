// Copyright 2014-2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr as std_ptr;
use std::slice;

use rawpointer::PointerExt;

use crate::imp_prelude::*;

use crate::arraytraits;
use crate::dimension;
use crate::dimension::IntoDimension;
use crate::dimension::{
    abs_index, axes_of, do_slice, merge_axes, size_of_shape_checked, stride_offset, Axes,
};
use crate::error::{self, ErrorKind, ShapeError};
use crate::itertools::zip;
use crate::zip::Zip;

use crate::iter::{
    AxisChunksIter, AxisChunksIterMut, AxisIter, AxisIterMut, ExactChunks, ExactChunksMut,
    IndexedIter, IndexedIterMut, Iter, IterMut, Lanes, LanesMut, Windows,
};
use crate::slice::MultiSlice;
use crate::stacking::concatenate;
use crate::{NdIndex, Slice, SliceInfo, SliceOrIndex};

/// # Methods For All Array Types
impl<A, S, D> ArrayBase<S, D>
where
    S: RawData<Elem = A>,
    D: Dimension,
{
    /// Return the total number of elements in the array.
    pub fn len(&self) -> usize {
        self.dim.size()
    }

    /// Return the length of `axis`.
    ///
    /// The axis should be in the range `Axis(` 0 .. *n* `)` where *n* is the
    /// number of dimensions (axes) of the array.
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn len_of(&self, axis: Axis) -> usize {
        self.dim[axis.index()]
    }

    /// Return whether the array has any elements
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the number of dimensions (axes) in the array
    pub fn ndim(&self) -> usize {
        self.dim.ndim()
    }

    /// Return the shape of the array in its “pattern” form,
    /// an integer in the one-dimensional case, tuple in the n-dimensional cases
    /// and so on.
    pub fn dim(&self) -> D::Pattern {
        self.dim.clone().into_pattern()
    }

    /// Return the shape of the array as it stored in the array.
    ///
    /// This is primarily useful for passing to other `ArrayBase`
    /// functions, such as when creating another array of the same
    /// shape and dimensionality.
    ///
    /// ```
    /// use ndarray::Array;
    ///
    /// let a = Array::from_elem((2, 3), 5.);
    ///
    /// // Create an array of zeros that's the same shape and dimensionality as `a`.
    /// let b = Array::<f64, _>::zeros(a.raw_dim());
    /// ```
    pub fn raw_dim(&self) -> D {
        self.dim.clone()
    }

    /// Return the shape of the array as a slice.
    ///
    /// Note that you probably don't want to use this to create an array of the
    /// same shape as another array because creating an array with e.g.
    /// [`Array::zeros()`](ArrayBase::zeros) using a shape of type `&[usize]`
    /// results in a dynamic-dimensional array. If you want to create an array
    /// that has the same shape and dimensionality as another array, use
    /// [`.raw_dim()`](ArrayBase::raw_dim) instead:
    ///
    /// ```rust
    /// use ndarray::{Array, Array2};
    ///
    /// let a = Array2::<i32>::zeros((3, 4));
    /// let shape = a.shape();
    /// assert_eq!(shape, &[3, 4]);
    ///
    /// // Since `a.shape()` returned `&[usize]`, we get an `ArrayD` instance:
    /// let b = Array::zeros(shape);
    /// assert_eq!(a.clone().into_dyn(), b);
    ///
    /// // To get the same dimension type, use `.raw_dim()` instead:
    /// let c = Array::zeros(a.raw_dim());
    /// assert_eq!(a, c);
    /// ```
    pub fn shape(&self) -> &[usize] {
        self.dim.slice()
    }

    /// Return the strides of the array as a slice.
    pub fn strides(&self) -> &[isize] {
        let s = self.strides.slice();
        // reinterpret unsigned integer as signed
        unsafe { slice::from_raw_parts(s.as_ptr() as *const _, s.len()) }
    }

    /// Return the stride of `axis`.
    ///
    /// The axis should be in the range `Axis(` 0 .. *n* `)` where *n* is the
    /// number of dimensions (axes) of the array.
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn stride_of(&self, axis: Axis) -> isize {
        // strides are reinterpreted as isize
        self.strides[axis.index()] as isize
    }

    /// Return a read-only view of the array
    pub fn view(&self) -> ArrayView<'_, A, D>
    where
        S: Data,
    {
        debug_assert!(self.pointer_is_inbounds());
        unsafe { ArrayView::new(self.ptr, self.dim.clone(), self.strides.clone()) }
    }

    /// Return a read-write view of the array
    pub fn view_mut(&mut self) -> ArrayViewMut<'_, A, D>
    where
        S: DataMut,
    {
        self.ensure_unique();
        unsafe { ArrayViewMut::new(self.ptr, self.dim.clone(), self.strides.clone()) }
    }

    /// Return an uniquely owned copy of the array.
    ///
    /// If the input array is contiguous and its strides are positive, then the
    /// output array will have the same memory layout. Otherwise, the layout of
    /// the output array is unspecified. If you need a particular layout, you
    /// can allocate a new array with the desired memory layout and
    /// [`.assign()`](#method.assign) the data. Alternatively, you can collect
    /// an iterator, like this for a result in standard layout:
    ///
    /// ```
    /// # use ndarray::prelude::*;
    /// # let arr = Array::from_shape_vec((2, 2).f(), vec![1, 2, 3, 4]).unwrap();
    /// # let owned = {
    /// Array::from_shape_vec(arr.raw_dim(), arr.iter().cloned().collect()).unwrap()
    /// # };
    /// # assert!(owned.is_standard_layout());
    /// # assert_eq!(arr, owned);
    /// ```
    ///
    /// or this for a result in column-major (Fortran) layout:
    ///
    /// ```
    /// # use ndarray::prelude::*;
    /// # let arr = Array::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap();
    /// # let owned = {
    /// Array::from_shape_vec(arr.raw_dim().f(), arr.t().iter().cloned().collect()).unwrap()
    /// # };
    /// # assert!(owned.t().is_standard_layout());
    /// # assert_eq!(arr, owned);
    /// ```
    pub fn to_owned(&self) -> Array<A, D>
    where
        A: Clone,
        S: Data,
    {
        if let Some(slc) = self.as_slice_memory_order() {
            unsafe {
                Array::from_shape_vec_unchecked(
                    self.dim.clone().strides(self.strides.clone()),
                    slc.to_vec(),
                )
            }
        } else {
            self.map(|x| x.clone())
        }
    }

    /// Return a shared ownership (copy on write) array.
    pub fn to_shared(&self) -> ArcArray<A, D>
    where
        A: Clone,
        S: Data,
    {
        // FIXME: Avoid copying if it’s already an ArcArray.
        self.to_owned().into_shared()
    }

    /// Turn the array into a uniquely owned array, cloning the array elements
    /// if necessary.
    pub fn into_owned(self) -> Array<A, D>
    where
        A: Clone,
        S: Data,
    {
        S::into_owned(self)
    }

    /// Turn the array into a shared ownership (copy on write) array,
    /// without any copying.
    pub fn into_shared(self) -> ArcArray<A, D>
    where
        S: DataOwned,
    {
        let data = self.data.into_shared();
        ArrayBase {
            data,
            ptr: self.ptr,
            dim: self.dim,
            strides: self.strides,
        }
    }

    /// Returns a reference to the first element of the array, or `None` if it
    /// is empty.
    pub fn first(&self) -> Option<&A>
    where
        S: Data,
    {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.as_ptr() })
        }
    }

    /// Returns a mutable reference to the first element of the array, or
    /// `None` if it is empty.
    pub fn first_mut(&mut self) -> Option<&mut A>
    where
        S: DataMut,
    {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &mut *self.as_mut_ptr() })
        }
    }

    /// Return an iterator of references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `&A`.
    pub fn iter(&self) -> Iter<'_, A, D>
    where
        S: Data,
    {
        debug_assert!(self.pointer_is_inbounds());
        self.view().into_iter_()
    }

    /// Return an iterator of mutable references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `&mut A`.
    pub fn iter_mut(&mut self) -> IterMut<'_, A, D>
    where
        S: DataMut,
    {
        self.view_mut().into_iter_()
    }

    /// Return an iterator of indexes and references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `(D::Pattern, &A)`.
    ///
    /// See also [`Zip::indexed`](struct.Zip.html)
    pub fn indexed_iter(&self) -> IndexedIter<'_, A, D>
    where
        S: Data,
    {
        IndexedIter::new(self.view().into_elements_base())
    }

    /// Return an iterator of indexes and mutable references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `(D::Pattern, &mut A)`.
    pub fn indexed_iter_mut(&mut self) -> IndexedIterMut<'_, A, D>
    where
        S: DataMut,
    {
        IndexedIterMut::new(self.view_mut().into_elements_base())
    }

    /// Return a sliced view of the array.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`SliceInfo`] and [`D::SliceArg`].
    ///
    /// [`SliceInfo`]: struct.SliceInfo.html
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `info` does not match the number of array axes.)
    pub fn slice<Do>(&self, info: &SliceInfo<D::SliceArg, Do>) -> ArrayView<'_, A, Do>
    where
        Do: Dimension,
        S: Data,
    {
        self.view().slice_move(info)
    }

    /// Return a sliced read-write view of the array.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`SliceInfo`] and [`D::SliceArg`].
    ///
    /// [`SliceInfo`]: struct.SliceInfo.html
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `info` does not match the number of array axes.)
    pub fn slice_mut<Do>(&mut self, info: &SliceInfo<D::SliceArg, Do>) -> ArrayViewMut<'_, A, Do>
    where
        Do: Dimension,
        S: DataMut,
    {
        self.view_mut().slice_move(info)
    }

    /// Return multiple disjoint, sliced, mutable views of the array.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`SliceInfo`] and [`D::SliceArg`].
    ///
    /// [`SliceInfo`]: struct.SliceInfo.html
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if any of the following occur:
    ///
    /// * if any of the views would intersect (i.e. if any element would appear in multiple slices)
    /// * if an index is out of bounds or step size is zero
    /// * if `D` is `IxDyn` and `info` does not match the number of array axes
    ///
    /// # Example
    ///
    /// ```
    /// use ndarray::{arr2, s};
    ///
    /// let mut a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    /// let (mut edges, mut middle) = a.multi_slice_mut((s![.., ..;2], s![.., 1]));
    /// edges.fill(1);
    /// middle.fill(0);
    /// assert_eq!(a, arr2(&[[1, 0, 1], [1, 0, 1]]));
    /// ```
    pub fn multi_slice_mut<'a, M>(&'a mut self, info: M) -> M::Output
    where
        M: MultiSlice<'a, A, D>,
        S: DataMut,
    {
        info.multi_slice_move(self.view_mut())
    }

    /// Slice the array, possibly changing the number of dimensions.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`SliceInfo`] and [`D::SliceArg`].
    ///
    /// [`SliceInfo`]: struct.SliceInfo.html
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `info` does not match the number of array axes.)
    pub fn slice_move<Do>(mut self, info: &SliceInfo<D::SliceArg, Do>) -> ArrayBase<S, Do>
    where
        Do: Dimension,
    {
        // Slice and collapse in-place without changing the number of dimensions.
        self.slice_collapse(&*info);

        let indices: &[SliceOrIndex] = (**info).as_ref();

        // Copy the dim and strides that remain after removing the subview axes.
        let out_ndim = info.out_ndim();
        let mut new_dim = Do::zeros(out_ndim);
        let mut new_strides = Do::zeros(out_ndim);
        izip!(self.dim.slice(), self.strides.slice(), indices)
            .filter_map(|(d, s, slice_or_index)| match slice_or_index {
                SliceOrIndex::Slice { .. } => Some((d, s)),
                SliceOrIndex::Index(_) => None,
            })
            .zip(izip!(new_dim.slice_mut(), new_strides.slice_mut()))
            .for_each(|((d, s), (new_d, new_s))| {
                *new_d = *d;
                *new_s = *s;
            });

        ArrayBase {
            ptr: self.ptr,
            data: self.data,
            dim: new_dim,
            strides: new_strides,
        }
    }

    /// Slice the array in place without changing the number of dimensions.
    ///
    /// Note that [`&SliceInfo`](struct.SliceInfo.html) (produced by the
    /// [`s![]`](macro.s!.html) macro) will usually coerce into `&D::SliceArg`
    /// automatically, but in some cases (e.g. if `D` is `IxDyn`), you may need
    /// to call `.as_ref()`.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`D::SliceArg`].
    ///
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `indices` does not match the number of array axes.)
    pub fn slice_collapse(&mut self, indices: &D::SliceArg) {
        let indices: &[SliceOrIndex] = indices.as_ref();
        assert_eq!(indices.len(), self.ndim());
        indices
            .iter()
            .enumerate()
            .for_each(|(axis, &slice_or_index)| match slice_or_index {
                SliceOrIndex::Slice { start, end, step } => {
                    self.slice_axis_inplace(Axis(axis), Slice { start, end, step })
                }
                SliceOrIndex::Index(index) => {
                    let i_usize = abs_index(self.len_of(Axis(axis)), index);
                    self.collapse_axis(Axis(axis), i_usize)
                }
            });
    }

    /// Return a view of the array, sliced along the specified axis.
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// **Panics** if `axis` is out of bounds.
    pub fn slice_axis(&self, axis: Axis, indices: Slice) -> ArrayView<'_, A, D>
    where
        S: Data,
    {
        let mut view = self.view();
        view.slice_axis_inplace(axis, indices);
        view
    }

    /// Return a mutable view of the array, sliced along the specified axis.
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// **Panics** if `axis` is out of bounds.
    pub fn slice_axis_mut(&mut self, axis: Axis, indices: Slice) -> ArrayViewMut<'_, A, D>
    where
        S: DataMut,
    {
        let mut view_mut = self.view_mut();
        view_mut.slice_axis_inplace(axis, indices);
        view_mut
    }

    /// Slice the array in place along the specified axis.
    ///
    /// **Panics** if an index is out of bounds or step size is zero.<br>
    /// **Panics** if `axis` is out of bounds.
    pub fn slice_axis_inplace(&mut self, axis: Axis, indices: Slice) {
        let offset = do_slice(
            &mut self.dim.slice_mut()[axis.index()],
            &mut self.strides.slice_mut()[axis.index()],
            indices,
        );
        unsafe {
            self.ptr = self.ptr.offset(offset);
        }
        debug_assert!(self.pointer_is_inbounds());
    }

    /// Return a reference to the element at `index`, or return `None`
    /// if the index is out of bounds.
    ///
    /// Arrays also support indexing syntax: `array[index]`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[1., 2.],
    ///                [3., 4.]]);
    ///
    /// assert!(
    ///     a.get((0, 1)) == Some(&2.) &&
    ///     a.get((0, 2)) == None &&
    ///     a[(0, 1)] == 2. &&
    ///     a[[0, 1]] == 2.
    /// );
    /// ```
    pub fn get<I>(&self, index: I) -> Option<&A>
    where
        I: NdIndex<D>,
        S: Data,
    {
        unsafe { self.get_ptr(index).map(|ptr| &*ptr) }
    }

    pub(crate) fn get_ptr<I>(&self, index: I) -> Option<*const A>
    where
        I: NdIndex<D>,
    {
        let ptr = self.ptr;
        index
            .index_checked(&self.dim, &self.strides)
            .map(move |offset| unsafe { ptr.as_ptr().offset(offset) as *const _ })
    }

    /// Return a mutable reference to the element at `index`, or return `None`
    /// if the index is out of bounds.
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut A>
    where
        S: DataMut,
        I: NdIndex<D>,
    {
        unsafe { self.get_ptr_mut(index).map(|ptr| &mut *ptr) }
    }

    pub(crate) fn get_ptr_mut<I>(&mut self, index: I) -> Option<*mut A>
    where
        S: RawDataMut,
        I: NdIndex<D>,
    {
        // const and mut are separate to enforce &mutness as well as the
        // extra code in as_mut_ptr
        let ptr = self.as_mut_ptr();
        index
            .index_checked(&self.dim, &self.strides)
            .map(move |offset| unsafe { ptr.offset(offset) })
    }

    /// Perform *unchecked* array indexing.
    ///
    /// Return a reference to the element at `index`.
    ///
    /// **Note:** only unchecked for non-debug builds of ndarray.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is in-bounds.
    #[inline]
    pub unsafe fn uget<I>(&self, index: I) -> &A
    where
        S: Data,
        I: NdIndex<D>,
    {
        arraytraits::debug_bounds_check(self, &index);
        let off = index.index_unchecked(&self.strides);
        &*self.ptr.as_ptr().offset(off)
    }

    /// Perform *unchecked* array indexing.
    ///
    /// Return a mutable reference to the element at `index`.
    ///
    /// **Note:** Only unchecked for non-debug builds of ndarray.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    ///
    /// 1. the index is in-bounds and
    ///
    /// 2. the data is uniquely held by the array. (This property is guaranteed
    ///    for `Array` and `ArrayViewMut`, but not for `ArcArray` or `CowArray`.)
    #[inline]
    pub unsafe fn uget_mut<I>(&mut self, index: I) -> &mut A
    where
        S: DataMut,
        I: NdIndex<D>,
    {
        debug_assert!(self.data.is_unique());
        arraytraits::debug_bounds_check(self, &index);
        let off = index.index_unchecked(&self.strides);
        &mut *self.ptr.as_ptr().offset(off)
    }

    /// Swap elements at indices `index1` and `index2`.
    ///
    /// Indices may be equal.
    ///
    /// ***Panics*** if an index is out of bounds.
    pub fn swap<I>(&mut self, index1: I, index2: I)
    where
        S: DataMut,
        I: NdIndex<D>,
    {
        let ptr1: *mut _ = &mut self[index1];
        let ptr2: *mut _ = &mut self[index2];
        unsafe {
            std_ptr::swap(ptr1, ptr2);
        }
    }

    /// Swap elements *unchecked* at indices `index1` and `index2`.
    ///
    /// Indices may be equal.
    ///
    /// **Note:** only unchecked for non-debug builds of ndarray.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    ///
    /// 1. both `index1 and `index2` are in-bounds and
    ///
    /// 2. the data is uniquely held by the array. (This property is guaranteed
    ///    for `Array` and `ArrayViewMut`, but not for `ArcArray` or `CowArray`.)
    pub unsafe fn uswap<I>(&mut self, index1: I, index2: I)
    where
        S: DataMut,
        I: NdIndex<D>,
    {
        debug_assert!(self.data.is_unique());
        arraytraits::debug_bounds_check(self, &index1);
        arraytraits::debug_bounds_check(self, &index2);
        let off1 = index1.index_unchecked(&self.strides);
        let off2 = index2.index_unchecked(&self.strides);
        std_ptr::swap(
            self.ptr.as_ptr().offset(off1),
            self.ptr.as_ptr().offset(off2),
        );
    }

    // `get` for zero-dimensional arrays
    // panics if dimension is not zero. otherwise an element is always present.
    fn get_0d(&self) -> &A
    where
        S: Data,
    {
        assert!(self.ndim() == 0);
        unsafe { &*self.as_ptr() }
    }

    /// Returns a view restricted to `index` along the axis, with the axis
    /// removed.
    ///
    /// See [*Subviews*](#subviews) for full documentation.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, ArrayView, Axis};
    ///
    /// let a = arr2(&[[1., 2. ],    // ... axis 0, row 0
    ///                [3., 4. ],    // --- axis 0, row 1
    ///                [5., 6. ]]);  // ... axis 0, row 2
    /// //               .   \
    /// //                .   axis 1, column 1
    /// //                 axis 1, column 0
    /// assert!(
    ///     a.index_axis(Axis(0), 1) == ArrayView::from(&[3., 4.]) &&
    ///     a.index_axis(Axis(1), 1) == ArrayView::from(&[2., 4., 6.])
    /// );
    /// ```
    pub fn index_axis(&self, axis: Axis, index: usize) -> ArrayView<'_, A, D::Smaller>
    where
        S: Data,
        D: RemoveAxis,
    {
        self.view().index_axis_move(axis, index)
    }

    /// Returns a mutable view restricted to `index` along the axis, with the
    /// axis removed.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, aview2, Axis};
    ///
    /// let mut a = arr2(&[[1., 2. ],
    ///                    [3., 4. ]]);
    /// //                   .   \
    /// //                    .   axis 1, column 1
    /// //                     axis 1, column 0
    ///
    /// {
    ///     let mut column1 = a.index_axis_mut(Axis(1), 1);
    ///     column1 += 10.;
    /// }
    ///
    /// assert!(
    ///     a == aview2(&[[1., 12.],
    ///                   [3., 14.]])
    /// );
    /// ```
    pub fn index_axis_mut(&mut self, axis: Axis, index: usize) -> ArrayViewMut<'_, A, D::Smaller>
    where
        S: DataMut,
        D: RemoveAxis,
    {
        self.view_mut().index_axis_move(axis, index)
    }

    /// Collapses the array to `index` along the axis and removes the axis.
    ///
    /// See [`.index_axis()`](#method.index_axis) and [*Subviews*](#subviews) for full documentation.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    pub fn index_axis_move(mut self, axis: Axis, index: usize) -> ArrayBase<S, D::Smaller>
    where
        D: RemoveAxis,
    {
        self.collapse_axis(axis, index);
        let dim = self.dim.remove_axis(axis);
        let strides = self.strides.remove_axis(axis);
        ArrayBase {
            ptr: self.ptr,
            data: self.data,
            dim,
            strides,
        }
    }

    /// Selects `index` along the axis, collapsing the axis into length one.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    pub fn collapse_axis(&mut self, axis: Axis, index: usize) {
        let offset = dimension::do_collapse_axis(&mut self.dim, &self.strides, axis.index(), index);
        self.ptr = unsafe { self.ptr.offset(offset) };
        debug_assert!(self.pointer_is_inbounds());
    }

    /// Along `axis`, select arbitrary subviews corresponding to `indices`
    /// and and copy them into a new array.
    ///
    /// **Panics** if `axis` or an element of `indices` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, Axis};
    ///
    /// let x = arr2(&[[0., 1.],
    ///                [2., 3.],
    ///                [4., 5.],
    ///                [6., 7.],
    ///                [8., 9.]]);
    ///
    /// let r = x.select(Axis(0), &[0, 4, 3]);
    /// assert!(
    ///         r == arr2(&[[0., 1.],
    ///                     [8., 9.],
    ///                     [6., 7.]])
    ///);
    /// ```
    pub fn select(&self, axis: Axis, indices: &[Ix]) -> Array<A, D>
    where
        A: Copy,
        S: Data,
        D: RemoveAxis,
    {
        let mut subs = vec![self.view(); indices.len()];
        for (&i, sub) in zip(indices, &mut subs[..]) {
            sub.collapse_axis(axis, i);
        }
        if subs.is_empty() {
            let mut dim = self.raw_dim();
            dim.set_axis(axis, 0);
            unsafe { Array::from_shape_vec_unchecked(dim, vec![]) }
        } else {
            concatenate(axis, &subs).unwrap()
        }
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// rows of the array. For a 2D array these are the regular rows.
    ///
    /// This is equivalent to `.lanes(Axis(n - 1))` where *n* is `self.ndim()`.
    ///
    /// For an array of dimensions *a* × *b* × *c* × ... × *l* × *m*
    /// it has *a* × *b* × *c* × ... × *l* rows each of length *m*.
    ///
    /// For example, in a 2 × 2 × 3 array, each row is 3 elements long
    /// and there are 2 × 2 = 4 rows in total.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, Axis, arr1};
    ///
    /// let a = arr3(&[[[ 0,  1,  2],    // -- row 0, 0
    ///                 [ 3,  4,  5]],   // -- row 0, 1
    ///                [[ 6,  7,  8],    // -- row 1, 0
    ///                 [ 9, 10, 11]]]); // -- row 1, 1
    ///
    /// // `genrows` will yield the four generalized rows of the array.
    /// for row in a.genrows() {
    ///     /* loop body */
    /// }
    /// ```
    pub fn genrows(&self) -> Lanes<'_, A, D::Smaller>
    where
        S: Data,
    {
        let mut n = self.ndim();
        if n == 0 {
            n += 1;
        }
        Lanes::new(self.view(), Axis(n - 1))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// rows of the array and yields mutable array views.
    ///
    /// Iterator element is `ArrayView1<A>` (1D read-write array view).
    pub fn genrows_mut(&mut self) -> LanesMut<'_, A, D::Smaller>
    where
        S: DataMut,
    {
        let mut n = self.ndim();
        if n == 0 {
            n += 1;
        }
        LanesMut::new(self.view_mut(), Axis(n - 1))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// columns of the array. For a 2D array these are the regular columns.
    ///
    /// This is equivalent to `.lanes(Axis(0))`.
    ///
    /// For an array of dimensions *a* × *b* × *c* × ... × *l* × *m*
    /// it has *b* × *c* × ... × *l* × *m* columns each of length *a*.
    ///
    /// For example, in a 2 × 2 × 3 array, each column is 2 elements long
    /// and there are 2 × 3 = 6 columns in total.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, Axis, arr1};
    ///
    /// // The generalized columns of a 3D array:
    /// // are directed along the 0th axis: 0 and 6, 1 and 7 and so on...
    /// let a = arr3(&[[[ 0,  1,  2], [ 3,  4,  5]],
    ///                [[ 6,  7,  8], [ 9, 10, 11]]]);
    ///
    /// // Here `gencolumns` will yield the six generalized columns of the array.
    /// for row in a.gencolumns() {
    ///     /* loop body */
    /// }
    /// ```
    pub fn gencolumns(&self) -> Lanes<'_, A, D::Smaller>
    where
        S: Data,
    {
        Lanes::new(self.view(), Axis(0))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// columns of the array and yields mutable array views.
    ///
    /// Iterator element is `ArrayView1<A>` (1D read-write array view).
    pub fn gencolumns_mut(&mut self) -> LanesMut<'_, A, D::Smaller>
    where
        S: DataMut,
    {
        LanesMut::new(self.view_mut(), Axis(0))
    }

    /// Return a producer and iterable that traverses over all 1D lanes
    /// pointing in the direction of `axis`.
    ///
    /// When the pointing in the direction of the first axis, they are *columns*,
    /// in the direction of the last axis *rows*; in general they are all
    /// *lanes* and are one dimensional.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, aview1, Axis};
    ///
    /// let a = arr3(&[[[ 0,  1,  2],
    ///                 [ 3,  4,  5]],
    ///                [[ 6,  7,  8],
    ///                 [ 9, 10, 11]]]);
    ///
    /// let inner0 = a.lanes(Axis(0));
    /// let inner1 = a.lanes(Axis(1));
    /// let inner2 = a.lanes(Axis(2));
    ///
    /// // The first lane for axis 0 is [0, 6]
    /// assert_eq!(inner0.into_iter().next().unwrap(), aview1(&[0, 6]));
    /// // The first lane for axis 1 is [0, 3]
    /// assert_eq!(inner1.into_iter().next().unwrap(), aview1(&[0, 3]));
    /// // The first lane for axis 2 is [0, 1, 2]
    /// assert_eq!(inner2.into_iter().next().unwrap(), aview1(&[0, 1, 2]));
    /// ```
    pub fn lanes(&self, axis: Axis) -> Lanes<'_, A, D::Smaller>
    where
        S: Data,
    {
        Lanes::new(self.view(), axis)
    }

    /// Return a producer and iterable that traverses over all 1D lanes
    /// pointing in the direction of `axis`.
    ///
    /// Iterator element is `ArrayViewMut1<A>` (1D read-write array view).
    pub fn lanes_mut(&mut self, axis: Axis) -> LanesMut<'_, A, D::Smaller>
    where
        S: DataMut,
    {
        LanesMut::new(self.view_mut(), axis)
    }

    /// Return an iterator that traverses over the outermost dimension
    /// and yields each subview.
    ///
    /// This is equivalent to `.axis_iter(Axis(0))`.
    ///
    /// Iterator element is `ArrayView<A, D::Smaller>` (read-only array view).
    #[allow(deprecated)]
    pub fn outer_iter(&self) -> AxisIter<'_, A, D::Smaller>
    where
        S: Data,
        D: RemoveAxis,
    {
        self.view().into_outer_iter()
    }

    /// Return an iterator that traverses over the outermost dimension
    /// and yields each subview.
    ///
    /// This is equivalent to `.axis_iter_mut(Axis(0))`.
    ///
    /// Iterator element is `ArrayViewMut<A, D::Smaller>` (read-write array view).
    #[allow(deprecated)]
    pub fn outer_iter_mut(&mut self) -> AxisIterMut<'_, A, D::Smaller>
    where
        S: DataMut,
        D: RemoveAxis,
    {
        self.view_mut().into_outer_iter()
    }

    /// Return an iterator that traverses over `axis`
    /// and yields each subview along it.
    ///
    /// For example, in a 3 × 4 × 5 array, with `axis` equal to `Axis(2)`,
    /// the iterator element
    /// is a 3 × 4 subview (and there are 5 in total), as shown
    /// in the picture below.
    ///
    /// Iterator element is `ArrayView<A, D::Smaller>` (read-only array view).
    ///
    /// See [*Subviews*](#subviews) for full documentation.
    ///
    /// **Panics** if `axis` is out of bounds.
    ///
    /// <img src="https://rust-ndarray.github.io/ndarray/images/axis_iter_3_4_5.svg" height="250px">
    pub fn axis_iter(&self, axis: Axis) -> AxisIter<'_, A, D::Smaller>
    where
        S: Data,
        D: RemoveAxis,
    {
        AxisIter::new(self.view(), axis)
    }

    /// Return an iterator that traverses over `axis`
    /// and yields each mutable subview along it.
    ///
    /// Iterator element is `ArrayViewMut<A, D::Smaller>`
    /// (read-write array view).
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn axis_iter_mut(&mut self, axis: Axis) -> AxisIterMut<'_, A, D::Smaller>
    where
        S: DataMut,
        D: RemoveAxis,
    {
        AxisIterMut::new(self.view_mut(), axis)
    }

    /// Return an iterator that traverses over `axis` by chunks of `size`,
    /// yielding non-overlapping views along that axis.
    ///
    /// Iterator element is `ArrayView<A, D>`
    ///
    /// The last view may have less elements if `size` does not divide
    /// the axis' dimension.
    ///
    /// **Panics** if `axis` is out of bounds or if `size` is zero.
    ///
    /// ```
    /// use ndarray::Array;
    /// use ndarray::{arr3, Axis};
    /// use std::iter::FromIterator;
    ///
    /// let a = Array::from_iter(0..28).into_shape((2, 7, 2)).unwrap();
    /// let mut iter = a.axis_chunks_iter(Axis(1), 2);
    ///
    /// // first iteration yields a 2 × 2 × 2 view
    /// assert_eq!(iter.next().unwrap(),
    ///            arr3(&[[[ 0,  1], [ 2, 3]],
    ///                   [[14, 15], [16, 17]]]));
    ///
    /// // however the last element is a 2 × 1 × 2 view since 7 % 2 == 1
    /// assert_eq!(iter.next_back().unwrap(), arr3(&[[[12, 13]],
    ///                                              [[26, 27]]]));
    /// ```
    pub fn axis_chunks_iter(&self, axis: Axis, size: usize) -> AxisChunksIter<'_, A, D>
    where
        S: Data,
    {
        AxisChunksIter::new(self.view(), axis, size)
    }

    /// Return an iterator that traverses over `axis` by chunks of `size`,
    /// yielding non-overlapping read-write views along that axis.
    ///
    /// Iterator element is `ArrayViewMut<A, D>`
    ///
    /// **Panics** if `axis` is out of bounds or if `size` is zero.
    pub fn axis_chunks_iter_mut(&mut self, axis: Axis, size: usize) -> AxisChunksIterMut<'_, A, D>
    where
        S: DataMut,
    {
        AxisChunksIterMut::new(self.view_mut(), axis, size)
    }

    /// Return an exact chunks producer (and iterable).
    ///
    /// It produces the whole chunks of a given n-dimensional chunk size,
    /// skipping the remainder along each dimension that doesn't fit evenly.
    ///
    /// The produced element is a `ArrayView<A, D>` with exactly the dimension
    /// `chunk_size`.
    ///
    /// **Panics** if any dimension of `chunk_size` is zero<br>
    /// (**Panics** if `D` is `IxDyn` and `chunk_size` does not match the
    /// number of array axes.)
    pub fn exact_chunks<E>(&self, chunk_size: E) -> ExactChunks<'_, A, D>
    where
        E: IntoDimension<Dim = D>,
        S: Data,
    {
        ExactChunks::new(self.view(), chunk_size)
    }

    /// Return an exact chunks producer (and iterable).
    ///
    /// It produces the whole chunks of a given n-dimensional chunk size,
    /// skipping the remainder along each dimension that doesn't fit evenly.
    ///
    /// The produced element is a `ArrayViewMut<A, D>` with exactly
    /// the dimension `chunk_size`.
    ///
    /// **Panics** if any dimension of `chunk_size` is zero<br>
    /// (**Panics** if `D` is `IxDyn` and `chunk_size` does not match the
    /// number of array axes.)
    ///
    /// ```rust
    /// use ndarray::Array;
    /// use ndarray::arr2;
    /// let mut a = Array::zeros((6, 7));
    ///
    /// // Fill each 2 × 2 chunk with the index of where it appeared in iteration
    /// for (i, mut chunk) in a.exact_chunks_mut((2, 2)).into_iter().enumerate() {
    ///     chunk.fill(i);
    /// }
    ///
    /// // The resulting array is:
    /// assert_eq!(
    ///   a,
    ///   arr2(&[[0, 0, 1, 1, 2, 2, 0],
    ///          [0, 0, 1, 1, 2, 2, 0],
    ///          [3, 3, 4, 4, 5, 5, 0],
    ///          [3, 3, 4, 4, 5, 5, 0],
    ///          [6, 6, 7, 7, 8, 8, 0],
    ///          [6, 6, 7, 7, 8, 8, 0]]));
    /// ```
    pub fn exact_chunks_mut<E>(&mut self, chunk_size: E) -> ExactChunksMut<'_, A, D>
    where
        E: IntoDimension<Dim = D>,
        S: DataMut,
    {
        ExactChunksMut::new(self.view_mut(), chunk_size)
    }

    /// Return a window producer and iterable.
    ///
    /// The windows are all distinct overlapping views of size `window_size`
    /// that fit into the array's shape.
    ///
    /// Will yield over no elements if window size is larger
    /// than the actual array size of any dimension.
    ///
    /// The produced element is an `ArrayView<A, D>` with exactly the dimension
    /// `window_size`.
    ///
    /// **Panics** if any dimension of `window_size` is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `window_size` does not match the
    /// number of array axes.)
    ///
    /// This is an illustration of the 2×2 windows in a 3×4 array:
    ///
    /// ```text
    ///          ──▶ Axis(1)
    ///
    ///      │   ┏━━━━━┳━━━━━┱─────┬─────┐   ┌─────┲━━━━━┳━━━━━┱─────┐   ┌─────┬─────┲━━━━━┳━━━━━┓
    ///      ▼   ┃ a₀₀ ┃ a₀₁ ┃     │     │   │     ┃ a₀₁ ┃ a₀₂ ┃     │   │     │     ┃ a₀₂ ┃ a₀₃ ┃
    /// Axis(0)  ┣━━━━━╋━━━━━╉─────┼─────┤   ├─────╊━━━━━╋━━━━━╉─────┤   ├─────┼─────╊━━━━━╋━━━━━┫
    ///          ┃ a₁₀ ┃ a₁₁ ┃     │     │   │     ┃ a₁₁ ┃ a₁₂ ┃     │   │     │     ┃ a₁₂ ┃ a₁₃ ┃
    ///          ┡━━━━━╇━━━━━╃─────┼─────┤   ├─────╄━━━━━╇━━━━━╃─────┤   ├─────┼─────╄━━━━━╇━━━━━┩
    ///          │     │     │     │     │   │     │     │     │     │   │     │     │     │     │
    ///          └─────┴─────┴─────┴─────┘   └─────┴─────┴─────┴─────┘   └─────┴─────┴─────┴─────┘
    ///
    ///          ┌─────┬─────┬─────┬─────┐   ┌─────┬─────┬─────┬─────┐   ┌─────┬─────┬─────┬─────┐
    ///          │     │     │     │     │   │     │     │     │     │   │     │     │     │     │
    ///          ┢━━━━━╈━━━━━╅─────┼─────┤   ├─────╆━━━━━╈━━━━━╅─────┤   ├─────┼─────╆━━━━━╈━━━━━┪
    ///          ┃ a₁₀ ┃ a₁₁ ┃     │     │   │     ┃ a₁₁ ┃ a₁₂ ┃     │   │     │     ┃ a₁₂ ┃ a₁₃ ┃
    ///          ┣━━━━━╋━━━━━╉─────┼─────┤   ├─────╊━━━━━╋━━━━━╉─────┤   ├─────┼─────╊━━━━━╋━━━━━┫
    ///          ┃ a₂₀ ┃ a₂₁ ┃     │     │   │     ┃ a₂₁ ┃ a₂₂ ┃     │   │     │     ┃ a₂₂ ┃ a₂₃ ┃
    ///          ┗━━━━━┻━━━━━┹─────┴─────┘   └─────┺━━━━━┻━━━━━┹─────┘   └─────┴─────┺━━━━━┻━━━━━┛
    /// ```
    pub fn windows<E>(&self, window_size: E) -> Windows<'_, A, D>
    where
        E: IntoDimension<Dim = D>,
        S: Data,
    {
        Windows::new(self.view(), window_size)
    }

    // Return (length, stride) for diagonal
    fn diag_params(&self) -> (Ix, Ixs) {
        /* empty shape has len 1 */
        let len = self.dim.slice().iter().cloned().min().unwrap_or(1);
        let stride = self.strides().iter().sum();
        (len, stride)
    }

    /// Return an view of the diagonal elements of the array.
    ///
    /// The diagonal is simply the sequence indexed by *(0, 0, .., 0)*,
    /// *(1, 1, ..., 1)* etc as long as all axes have elements.
    pub fn diag(&self) -> ArrayView1<'_, A>
    where
        S: Data,
    {
        self.view().into_diag()
    }

    /// Return a read-write view over the diagonal elements of the array.
    pub fn diag_mut(&mut self) -> ArrayViewMut1<'_, A>
    where
        S: DataMut,
    {
        self.view_mut().into_diag()
    }

    /// Return the diagonal as a one-dimensional array.
    pub fn into_diag(self) -> ArrayBase<S, Ix1> {
        let (len, stride) = self.diag_params();
        ArrayBase {
            data: self.data,
            ptr: self.ptr,
            dim: Ix1(len),
            strides: Ix1(stride as Ix),
        }
    }

    /// Try to make the array unshared.
    ///
    /// This is equivalent to `.ensure_unique()` if `S: DataMut`.
    ///
    /// This method is mostly only useful with unsafe code.
    fn try_ensure_unique(&mut self)
    where
        S: RawDataMut,
    {
        debug_assert!(self.pointer_is_inbounds());
        S::try_ensure_unique(self);
        debug_assert!(self.pointer_is_inbounds());
    }

    /// Make the array unshared.
    ///
    /// This method is mostly only useful with unsafe code.
    fn ensure_unique(&mut self)
    where
        S: DataMut,
    {
        debug_assert!(self.pointer_is_inbounds());
        S::ensure_unique(self);
        debug_assert!(self.pointer_is_inbounds());
    }

    /// Return `true` if the array data is laid out in contiguous “C order” in
    /// memory (where the last index is the most rapidly varying).
    ///
    /// Return `false` otherwise, i.e the array is possibly not
    /// contiguous in memory, it has custom strides, etc.
    pub fn is_standard_layout(&self) -> bool {
        fn is_standard_layout<D: Dimension>(dim: &D, strides: &D) -> bool {
            if let Some(1) = D::NDIM {
                return strides[0] == 1 || dim[0] <= 1;
            }
            if dim.slice().iter().any(|&d| d == 0) {
                return true;
            }
            let defaults = dim.default_strides();
            // check all dimensions -- a dimension of length 1 can have unequal strides
            for (&dim, &s, &ds) in izip!(dim.slice(), strides.slice(), defaults.slice()) {
                if dim != 1 && s != ds {
                    return false;
                }
            }
            true
        }
        is_standard_layout(&self.dim, &self.strides)
    }

    /// Return true if the array is known to be contiguous.
    ///
    /// Will detect c- and f-contig arrays correctly, but otherwise
    /// There are some false negatives.
    pub(crate) fn is_contiguous(&self) -> bool {
        D::is_contiguous(&self.dim, &self.strides)
    }

    /// Return a standard-layout array containing the data, cloning if
    /// necessary.
    ///
    /// If `self` is in standard layout, a COW view of the data is returned
    /// without cloning. Otherwise, the data is cloned, and the returned array
    /// owns the cloned data.
    ///
    /// ```
    /// use ndarray::Array2;
    ///
    /// let standard = Array2::<f64>::zeros((3, 4));
    /// assert!(standard.is_standard_layout());
    /// let cow_view = standard.as_standard_layout();
    /// assert!(cow_view.is_view());
    /// assert!(cow_view.is_standard_layout());
    ///
    /// let fortran = standard.reversed_axes();
    /// assert!(!fortran.is_standard_layout());
    /// let cow_owned = fortran.as_standard_layout();
    /// assert!(cow_owned.is_owned());
    /// assert!(cow_owned.is_standard_layout());
    /// ```
    pub fn as_standard_layout(&self) -> CowArray<'_, A, D>
    where
        S: Data<Elem = A>,
        A: Clone,
    {
        if self.is_standard_layout() {
            CowArray::from(self.view())
        } else {
            let v: Vec<A> = self.iter().cloned().collect();
            let dim = self.dim.clone();
            assert_eq!(v.len(), dim.size());
            let owned_array: Array<A, D> = unsafe {
                // Safe because the shape and element type are from the existing array
                // and the strides are the default strides.
                Array::from_shape_vec_unchecked(dim, v)
            };
            CowArray::from(owned_array)
        }
    }

    /// Return a pointer to the first element in the array.
    ///
    /// Raw access to array elements needs to follow the strided indexing
    /// scheme: an element at multi-index *I* in an array with strides *S* is
    /// located at offset
    ///
    /// *Σ<sub>0 ≤ k < d</sub> I<sub>k</sub> × S<sub>k</sub>*
    ///
    /// where *d* is `self.ndim()`.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const A {
        self.ptr.as_ptr() as *const A
    }

    /// Return a mutable pointer to the first element in the array.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut A
    where
        S: RawDataMut,
    {
        self.try_ensure_unique(); // for ArcArray
        self.ptr.as_ptr()
    }

    /// Return a raw view of the array.
    #[inline]
    pub fn raw_view(&self) -> RawArrayView<A, D> {
        unsafe { RawArrayView::new(self.ptr, self.dim.clone(), self.strides.clone()) }
    }

    /// Return a raw mutable view of the array.
    #[inline]
    pub fn raw_view_mut(&mut self) -> RawArrayViewMut<A, D>
    where
        S: RawDataMut,
    {
        self.try_ensure_unique(); // for ArcArray
        unsafe { RawArrayViewMut::new(self.ptr, self.dim.clone(), self.strides.clone()) }
    }

    /// Return the array’s data as a slice, if it is contiguous and in standard order.
    /// Return `None` otherwise.
    ///
    /// If this function returns `Some(_)`, then the element order in the slice
    /// corresponds to the logical order of the array’s elements.
    pub fn as_slice(&self) -> Option<&[A]>
    where
        S: Data,
    {
        if self.is_standard_layout() {
            unsafe { Some(slice::from_raw_parts(self.ptr.as_ptr(), self.len())) }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice, if it is contiguous and in standard order.
    /// Return `None` otherwise.
    pub fn as_slice_mut(&mut self) -> Option<&mut [A]>
    where
        S: DataMut,
    {
        if self.is_standard_layout() {
            self.ensure_unique();
            unsafe { Some(slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len())) }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice if it is contiguous,
    /// return `None` otherwise.
    ///
    /// If this function returns `Some(_)`, then the elements in the slice
    /// have whatever order the elements have in memory.
    ///
    /// Implementation notes: Does not yet support negatively strided arrays.
    pub fn as_slice_memory_order(&self) -> Option<&[A]>
    where
        S: Data,
    {
        if self.is_contiguous() {
            unsafe { Some(slice::from_raw_parts(self.ptr.as_ptr(), self.len())) }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice if it is contiguous,
    /// return `None` otherwise.
    pub fn as_slice_memory_order_mut(&mut self) -> Option<&mut [A]>
    where
        S: DataMut,
    {
        if self.is_contiguous() {
            self.ensure_unique();
            unsafe { Some(slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len())) }
        } else {
            None
        }
    }

    /// Transform the array into `shape`; any shape with the same number of
    /// elements is accepted, but the source array or view must be in standard
    /// or column-major (Fortran) layout.
    ///
    /// **Errors** if the shapes don't have the same number of elements.<br>
    /// **Errors** if the input array is not c- or f-contiguous.
    ///
    /// ```
    /// use ndarray::{aview1, aview2};
    ///
    /// assert!(
    ///     aview1(&[1., 2., 3., 4.]).into_shape((2, 2)).unwrap()
    ///     == aview2(&[[1., 2.],
    ///                 [3., 4.]])
    /// );
    /// ```
    pub fn into_shape<E>(self, shape: E) -> Result<ArrayBase<S, E::Dim>, ShapeError>
    where
        E: IntoDimension,
    {
        let shape = shape.into_dimension();
        if size_of_shape_checked(&shape) != Ok(self.dim.size()) {
            return Err(error::incompatible_shapes(&self.dim, &shape));
        }
        // Check if contiguous, if not => copy all, else just adapt strides
        if self.is_standard_layout() {
            Ok(ArrayBase {
                data: self.data,
                ptr: self.ptr,
                strides: shape.default_strides(),
                dim: shape,
            })
        } else if self.ndim() > 1 && self.raw_view().reversed_axes().is_standard_layout() {
            Ok(ArrayBase {
                data: self.data,
                ptr: self.ptr,
                strides: shape.fortran_strides(),
                dim: shape,
            })
        } else {
            Err(error::from_kind(error::ErrorKind::IncompatibleLayout))
        }
    }

    /// *Note: Reshape is for `ArcArray` only. Use `.into_shape()` for
    /// other arrays and array views.*
    ///
    /// Transform the array into `shape`; any shape with the same number of
    /// elements is accepted.
    ///
    /// May clone all elements if needed to arrange elements in standard
    /// layout (and break sharing).
    ///
    /// **Panics** if shapes are incompatible.
    ///
    /// ```
    /// use ndarray::{rcarr1, rcarr2};
    ///
    /// assert!(
    ///     rcarr1(&[1., 2., 3., 4.]).reshape((2, 2))
    ///     == rcarr2(&[[1., 2.],
    ///                 [3., 4.]])
    /// );
    /// ```
    pub fn reshape<E>(&self, shape: E) -> ArrayBase<S, E::Dim>
    where
        S: DataShared + DataOwned,
        A: Clone,
        E: IntoDimension,
    {
        let shape = shape.into_dimension();
        if size_of_shape_checked(&shape) != Ok(self.dim.size()) {
            panic!(
                "ndarray: incompatible shapes in reshape, attempted from: {:?}, to: {:?}",
                self.dim.slice(),
                shape.slice()
            )
        }
        // Check if contiguous, if not => copy all, else just adapt strides
        if self.is_standard_layout() {
            let cl = self.clone();
            ArrayBase {
                data: cl.data,
                ptr: cl.ptr,
                strides: shape.default_strides(),
                dim: shape,
            }
        } else {
            let v = self.iter().cloned().collect::<Vec<A>>();
            unsafe { ArrayBase::from_shape_vec_unchecked(shape, v) }
        }
    }

    /// Convert any array or array view to a dynamic dimensional array or
    /// array view (respectively).
    ///
    /// ```
    /// use ndarray::{arr2, ArrayD};
    ///
    /// let array: ArrayD<i32> = arr2(&[[1, 2],
    ///                                 [3, 4]]).into_dyn();
    /// ```
    pub fn into_dyn(self) -> ArrayBase<S, IxDyn> {
        ArrayBase {
            data: self.data,
            ptr: self.ptr,
            dim: self.dim.into_dyn(),
            strides: self.strides.into_dyn(),
        }
    }

    /// Convert an array or array view to another with the same type, but
    /// different dimensionality type. Errors if the dimensions don't agree.
    ///
    /// ```
    /// use ndarray::{ArrayD, Ix2, IxDyn};
    ///
    /// // Create a dynamic dimensionality array and convert it to an Array2
    /// // (Ix2 dimension type).
    ///
    /// let array = ArrayD::<f64>::zeros(IxDyn(&[10, 10]));
    ///
    /// assert!(array.into_dimensionality::<Ix2>().is_ok());
    /// ```
    pub fn into_dimensionality<D2>(self) -> Result<ArrayBase<S, D2>, ShapeError>
    where
        D2: Dimension,
    {
        if let Some(dim) = D2::from_dimension(&self.dim) {
            if let Some(strides) = D2::from_dimension(&self.strides) {
                return Ok(ArrayBase {
                    data: self.data,
                    ptr: self.ptr,
                    dim,
                    strides,
                });
            }
        }
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape))
    }

    /// Act like a larger size and/or shape array by *broadcasting*
    /// into a larger shape, if possible.
    ///
    /// Return `None` if shapes can not be broadcast together.
    ///
    /// ***Background***
    ///
    ///  * Two axes are compatible if they are equal, or one of them is 1.
    ///  * In this instance, only the axes of the smaller side (self) can be 1.
    ///
    /// Compare axes beginning with the *last* axis of each shape.
    ///
    /// For example (1, 2, 4) can be broadcast into (7, 6, 2, 4)
    /// because its axes are either equal or 1 (or missing);
    /// while (2, 2) can *not* be broadcast into (2, 4).
    ///
    /// The implementation creates a view with strides set to zero for the
    /// axes that are to be repeated.
    ///
    /// The broadcasting documentation for Numpy has more information.
    ///
    /// ```
    /// use ndarray::{aview1, aview2};
    ///
    /// assert!(
    ///     aview1(&[1., 0.]).broadcast((10, 2)).unwrap()
    ///     == aview2(&[[1., 0.]; 10])
    /// );
    /// ```
    pub fn broadcast<E>(&self, dim: E) -> Option<ArrayView<'_, A, E::Dim>>
    where
        E: IntoDimension,
        S: Data,
    {
        /// Return new stride when trying to grow `from` into shape `to`
        ///
        /// Broadcasting works by returning a "fake stride" where elements
        /// to repeat are in axes with 0 stride, so that several indexes point
        /// to the same element.
        ///
        /// **Note:** Cannot be used for mutable iterators, since repeating
        /// elements would create aliasing pointers.
        fn upcast<D: Dimension, E: Dimension>(to: &D, from: &E, stride: &E) -> Option<D> {
            // Make sure the product of non-zero axis lengths does not exceed
            // `isize::MAX`. This is the only safety check we need to perform
            // because all the other constraints of `ArrayBase` are guaranteed
            // to be met since we're starting from a valid `ArrayBase`.
            let _ = size_of_shape_checked(to).ok()?;

            let mut new_stride = to.clone();
            // begin at the back (the least significant dimension)
            // size of the axis has to either agree or `from` has to be 1
            if to.ndim() < from.ndim() {
                return None;
            }

            {
                let mut new_stride_iter = new_stride.slice_mut().iter_mut().rev();
                for ((er, es), dr) in from
                    .slice()
                    .iter()
                    .rev()
                    .zip(stride.slice().iter().rev())
                    .zip(new_stride_iter.by_ref())
                {
                    /* update strides */
                    if *dr == *er {
                        /* keep stride */
                        *dr = *es;
                    } else if *er == 1 {
                        /* dead dimension, zero stride */
                        *dr = 0
                    } else {
                        return None;
                    }
                }

                /* set remaining strides to zero */
                for dr in new_stride_iter {
                    *dr = 0;
                }
            }
            Some(new_stride)
        }
        let dim = dim.into_dimension();

        // Note: zero strides are safe precisely because we return an read-only view
        let broadcast_strides = match upcast(&dim, &self.dim, &self.strides) {
            Some(st) => st,
            None => return None,
        };
        unsafe { Some(ArrayView::new(self.ptr, dim, broadcast_strides)) }
    }

    /// Swap axes `ax` and `bx`.
    ///
    /// This does not move any data, it just adjusts the array’s dimensions
    /// and strides.
    ///
    /// **Panics** if the axes are out of bounds.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let mut a = arr2(&[[1., 2., 3.]]);
    /// a.swap_axes(0, 1);
    /// assert!(
    ///     a == arr2(&[[1.], [2.], [3.]])
    /// );
    /// ```
    pub fn swap_axes(&mut self, ax: usize, bx: usize) {
        self.dim.slice_mut().swap(ax, bx);
        self.strides.slice_mut().swap(ax, bx);
    }

    /// Permute the axes.
    ///
    /// This does not move any data, it just adjusts the array’s dimensions
    /// and strides.
    ///
    /// *i* in the *j*-th place in the axes sequence means `self`'s *i*-th axis
    /// becomes `self.permuted_axes()`'s *j*-th axis
    ///
    /// **Panics** if any of the axes are out of bounds, if an axis is missing,
    /// or if an axis is repeated more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// use ndarray::{arr2, Array3};
    ///
    /// let a = arr2(&[[0, 1], [2, 3]]);
    /// assert_eq!(a.view().permuted_axes([1, 0]), a.t());
    ///
    /// let b = Array3::<u8>::zeros((1, 2, 3));
    /// assert_eq!(b.permuted_axes([1, 0, 2]).shape(), &[2, 1, 3]);
    /// ```
    pub fn permuted_axes<T>(self, axes: T) -> ArrayBase<S, D>
    where
        T: IntoDimension<Dim = D>,
    {
        let axes = axes.into_dimension();
        // Ensure that each axis is used exactly once.
        let mut usage_counts = D::zeros(self.ndim());
        for axis in axes.slice() {
            usage_counts[*axis] += 1;
        }
        for count in usage_counts.slice() {
            assert_eq!(*count, 1, "each axis must be listed exactly once");
        }
        // Determine the new shape and strides.
        let mut new_dim = usage_counts; // reuse to avoid an allocation
        let mut new_strides = D::zeros(self.ndim());
        {
            let dim = self.dim.slice();
            let strides = self.strides.slice();
            for (new_axis, &axis) in axes.slice().iter().enumerate() {
                new_dim[new_axis] = dim[axis];
                new_strides[new_axis] = strides[axis];
            }
        }
        ArrayBase {
            dim: new_dim,
            strides: new_strides,
            ..self
        }
    }

    /// Transpose the array by reversing axes.
    ///
    /// Transposition reverses the order of the axes (dimensions and strides)
    /// while retaining the same data.
    pub fn reversed_axes(mut self) -> ArrayBase<S, D> {
        self.dim.slice_mut().reverse();
        self.strides.slice_mut().reverse();
        self
    }

    /// Return a transposed view of the array.
    ///
    /// This is a shorthand for `self.view().reversed_axes()`.
    ///
    /// See also the more general methods `.reversed_axes()` and `.swap_axes()`.
    pub fn t(&self) -> ArrayView<'_, A, D>
    where
        S: Data,
    {
        self.view().reversed_axes()
    }

    /// Return an iterator over the length and stride of each axis.
    pub fn axes(&self) -> Axes<'_, D> {
        axes_of(&self.dim, &self.strides)
    }

    /*
    /// Return the axis with the least stride (by absolute value)
    pub fn min_stride_axis(&self) -> Axis {
        self.dim.min_stride_axis(&self.strides)
    }
    */

    /// Return the axis with the greatest stride (by absolute value),
    /// preferring axes with len > 1.
    pub fn max_stride_axis(&self) -> Axis {
        self.dim.max_stride_axis(&self.strides)
    }

    /// Reverse the stride of `axis`.
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn invert_axis(&mut self, axis: Axis) {
        unsafe {
            let s = self.strides.axis(axis) as Ixs;
            let m = self.dim.axis(axis);
            if m != 0 {
                self.ptr = self.ptr.offset(stride_offset(m - 1, s as Ix));
            }
            self.strides.set_axis(axis, (-s) as Ix);
        }
    }

    /// If possible, merge in the axis `take` to `into`.
    ///
    /// Returns `true` iff the axes are now merged.
    ///
    /// This method merges the axes if movement along the two original axes
    /// (moving fastest along the `into` axis) can be equivalently represented
    /// as movement along one (merged) axis. Merging the axes preserves this
    /// order in the merged axis. If `take` and `into` are the same axis, then
    /// the axis is "merged" if its length is ≤ 1.
    ///
    /// If the return value is `true`, then the following hold:
    ///
    /// * The new length of the `into` axis is the product of the original
    ///   lengths of the two axes.
    ///
    /// * The new length of the `take` axis is 0 if the product of the original
    ///   lengths of the two axes is 0, and 1 otherwise.
    ///
    /// If the return value is `false`, then merging is not possible, and the
    /// original shape and strides have been preserved.
    ///
    /// Note that the ordering constraint means that if it's possible to merge
    /// `take` into `into`, it's usually not possible to merge `into` into
    /// `take`, and vice versa.
    ///
    /// ```
    /// use ndarray::Array3;
    /// use ndarray::Axis;
    ///
    /// let mut a = Array3::<f64>::zeros((2, 3, 4));
    /// assert!(a.merge_axes(Axis(1), Axis(2)));
    /// assert_eq!(a.shape(), &[2, 1, 12]);
    /// ```
    ///
    /// ***Panics*** if an axis is out of bounds.
    pub fn merge_axes(&mut self, take: Axis, into: Axis) -> bool {
        merge_axes(&mut self.dim, &mut self.strides, take, into)
    }

    /// Insert new array axis at `axis` and return the result.
    ///
    /// ```
    /// use ndarray::{Array3, Axis, arr1, arr2};
    ///
    /// // Convert a 1-D array into a row vector (2-D).
    /// let a = arr1(&[1, 2, 3]);
    /// let row = a.insert_axis(Axis(0));
    /// assert_eq!(row, arr2(&[[1, 2, 3]]));
    ///
    /// // Convert a 1-D array into a column vector (2-D).
    /// let b = arr1(&[1, 2, 3]);
    /// let col = b.insert_axis(Axis(1));
    /// assert_eq!(col, arr2(&[[1], [2], [3]]));
    ///
    /// // The new axis always has length 1.
    /// let b = Array3::<f64>::zeros((3, 4, 5));
    /// assert_eq!(b.insert_axis(Axis(2)).shape(), &[3, 4, 1, 5]);
    /// ```
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn insert_axis(self, axis: Axis) -> ArrayBase<S, D::Larger> {
        assert!(axis.index() <= self.ndim());
        let ArrayBase {
            ptr,
            data,
            dim,
            strides,
        } = self;
        ArrayBase {
            ptr,
            data,
            dim: dim.insert_axis(axis),
            strides: strides.insert_axis(axis),
        }
    }

    /// Remove array axis `axis` and return the result.
    ///
    /// This is equivalent to `.index-axis_move(axis, 0)` and makes most sense to use if the
    /// axis to remove is of length 1.
    ///
    /// **Panics** if the axis is out of bounds or its length is zero.
    pub fn remove_axis(self, axis: Axis) -> ArrayBase<S, D::Smaller>
    where
        D: RemoveAxis,
    {
        self.index_axis_move(axis, 0)
    }

    fn pointer_is_inbounds(&self) -> bool {
        match self.data._data_slice() {
            None => {
                // special case for non-owned views
                true
            }
            Some(slc) => {
                let ptr = slc.as_ptr() as *mut A;
                let end = unsafe { ptr.add(slc.len()) };
                self.ptr.as_ptr() >= ptr && self.ptr.as_ptr() <= end
            }
        }
    }

    /// Perform an elementwise assigment to `self` from `rhs`.
    ///
    /// If their shapes disagree, `rhs` is broadcast to the shape of `self`.
    ///
    /// **Panics** if broadcasting isn’t possible.
    pub fn assign<E: Dimension, S2>(&mut self, rhs: &ArrayBase<S2, E>)
    where
        S: DataMut,
        A: Clone,
        S2: Data<Elem = A>,
    {
        self.zip_mut_with(rhs, |x, y| *x = y.clone());
    }

    /// Perform an elementwise assigment to `self` from element `x`.
    pub fn fill(&mut self, x: A)
    where
        S: DataMut,
        A: Clone,
    {
        self.unordered_foreach_mut(move |elt| *elt = x.clone());
    }

    fn zip_mut_with_same_shape<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, mut f: F)
    where
        S: DataMut,
        S2: Data<Elem = B>,
        E: Dimension,
        F: FnMut(&mut A, &B),
    {
        debug_assert_eq!(self.shape(), rhs.shape());

        if self.dim.strides_equivalent(&self.strides, &rhs.strides) {
            if let Some(self_s) = self.as_slice_memory_order_mut() {
                if let Some(rhs_s) = rhs.as_slice_memory_order() {
                    for (s, r) in self_s.iter_mut().zip(rhs_s) {
                        f(s, &r);
                    }
                    return;
                }
            }
        }

        // Otherwise, fall back to the outer iter
        self.zip_mut_with_by_rows(rhs, f);
    }

    // zip two arrays where they have different layout or strides
    #[inline(always)]
    fn zip_mut_with_by_rows<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, mut f: F)
    where
        S: DataMut,
        S2: Data<Elem = B>,
        E: Dimension,
        F: FnMut(&mut A, &B),
    {
        debug_assert_eq!(self.shape(), rhs.shape());
        debug_assert_ne!(self.ndim(), 0);

        // break the arrays up into their inner rows
        let n = self.ndim();
        let dim = self.raw_dim();
        Zip::from(LanesMut::new(self.view_mut(), Axis(n - 1)))
            .and(Lanes::new(rhs.broadcast_assume(dim), Axis(n - 1)))
            .apply(move |s_row, r_row| Zip::from(s_row).and(r_row).apply(|a, b| f(a, b)));
    }

    fn zip_mut_with_elem<B, F>(&mut self, rhs_elem: &B, mut f: F)
    where
        S: DataMut,
        F: FnMut(&mut A, &B),
    {
        self.unordered_foreach_mut(move |elt| f(elt, rhs_elem));
    }

    /// Traverse two arrays in unspecified order, in lock step,
    /// calling the closure `f` on each element pair.
    ///
    /// If their shapes disagree, `rhs` is broadcast to the shape of `self`.
    ///
    /// **Panics** if broadcasting isn’t possible.
    #[inline]
    pub fn zip_mut_with<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, f: F)
    where
        S: DataMut,
        S2: Data<Elem = B>,
        E: Dimension,
        F: FnMut(&mut A, &B),
    {
        if rhs.dim.ndim() == 0 {
            // Skip broadcast from 0-dim array
            self.zip_mut_with_elem(rhs.get_0d(), f);
        } else if self.dim.ndim() == rhs.dim.ndim() && self.shape() == rhs.shape() {
            self.zip_mut_with_same_shape(rhs, f);
        } else {
            let rhs_broadcast = rhs.broadcast_unwrap(self.raw_dim());
            self.zip_mut_with_by_rows(&rhs_broadcast, f);
        }
    }

    /// Traverse the array elements and apply a fold,
    /// returning the resulting value.
    ///
    /// Elements are visited in arbitrary order.
    pub fn fold<'a, F, B>(&'a self, init: B, f: F) -> B
    where
        F: FnMut(B, &'a A) -> B,
        A: 'a,
        S: Data,
    {
        if let Some(slc) = self.as_slice_memory_order() {
            slc.iter().fold(init, f)
        } else {
            let mut v = self.view();
            // put the narrowest axis at the last position
            match v.ndim() {
                0 | 1 => {}
                2 => {
                    if self.len_of(Axis(1)) <= 1
                        || self.len_of(Axis(0)) > 1
                            && self.stride_of(Axis(0)).abs() < self.stride_of(Axis(1)).abs()
                    {
                        v.swap_axes(0, 1);
                    }
                }
                n => {
                    let last = n - 1;
                    let narrow_axis = v
                        .axes()
                        .filter(|ax| ax.len() > 1)
                        .min_by_key(|ax| ax.stride().abs())
                        .map_or(last, |ax| ax.axis().index());
                    v.swap_axes(last, narrow_axis);
                }
            }
            v.into_elements_base().fold(init, f)
        }
    }

    /// Call `f` by reference on each element and create a new array
    /// with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return an array with the same shape as `self`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[ 0., 1.],
    ///                [-1., 2.]]);
    /// assert!(
    ///     a.map(|x| *x >= 1.0)
    ///     == arr2(&[[false, true],
    ///               [false, true]])
    /// );
    /// ```
    pub fn map<'a, B, F>(&'a self, f: F) -> Array<B, D>
    where
        F: FnMut(&'a A) -> B,
        A: 'a,
        S: Data,
    {
        if let Some(slc) = self.as_slice_memory_order() {
            let v = crate::iterators::to_vec_mapped(slc.iter(), f);
            unsafe {
                ArrayBase::from_shape_vec_unchecked(
                    self.dim.clone().strides(self.strides.clone()),
                    v,
                )
            }
        } else {
            let v = crate::iterators::to_vec_mapped(self.iter(), f);
            unsafe { ArrayBase::from_shape_vec_unchecked(self.dim.clone(), v) }
        }
    }

    /// Call `f` on a mutable reference of each element and create a new array
    /// with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return an array with the same shape as `self`.
    pub fn map_mut<'a, B, F>(&'a mut self, f: F) -> Array<B, D>
    where
        F: FnMut(&'a mut A) -> B,
        A: 'a,
        S: DataMut,
    {
        let dim = self.dim.clone();
        if self.is_contiguous() {
            let strides = self.strides.clone();
            let slc = self.as_slice_memory_order_mut().unwrap();
            let v = crate::iterators::to_vec_mapped(slc.iter_mut(), f);
            unsafe { ArrayBase::from_shape_vec_unchecked(dim.strides(strides), v) }
        } else {
            let v = crate::iterators::to_vec_mapped(self.iter_mut(), f);
            unsafe { ArrayBase::from_shape_vec_unchecked(dim, v) }
        }
    }

    /// Call `f` by **v**alue on each element and create a new array
    /// with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return an array with the same shape as `self`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[ 0., 1.],
    ///                [-1., 2.]]);
    /// assert!(
    ///     a.mapv(f32::abs) == arr2(&[[0., 1.],
    ///                                [1., 2.]])
    /// );
    /// ```
    pub fn mapv<B, F>(&self, mut f: F) -> Array<B, D>
    where
        F: FnMut(A) -> B,
        A: Clone,
        S: Data,
    {
        self.map(move |x| f(x.clone()))
    }

    /// Call `f` by **v**alue on each element, update the array with the new values
    /// and return it.
    ///
    /// Elements are visited in arbitrary order.
    pub fn mapv_into<F>(mut self, f: F) -> Self
    where
        S: DataMut,
        F: FnMut(A) -> A,
        A: Clone,
    {
        self.mapv_inplace(f);
        self
    }

    /// Modify the array in place by calling `f` by mutable reference on each element.
    ///
    /// Elements are visited in arbitrary order.
    pub fn map_inplace<F>(&mut self, f: F)
    where
        S: DataMut,
        F: FnMut(&mut A),
    {
        self.unordered_foreach_mut(f);
    }

    /// Modify the array in place by calling `f` by **v**alue on each element.
    /// The array is updated with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use ndarray::arr2;
    ///
    /// # #[cfg(feature = "approx")] {
    /// let mut a = arr2(&[[ 0., 1.],
    ///                    [-1., 2.]]);
    /// a.mapv_inplace(f32::exp);
    /// assert_abs_diff_eq!(
    ///     a,
    ///     arr2(&[[1.00000, 2.71828],
    ///            [0.36788, 7.38906]]),
    ///     epsilon = 1e-5,
    /// );
    /// # }
    /// ```
    pub fn mapv_inplace<F>(&mut self, mut f: F)
    where
        S: DataMut,
        F: FnMut(A) -> A,
        A: Clone,
    {
        self.unordered_foreach_mut(move |x| *x = f(x.clone()));
    }

    /// Visit each element in the array by calling `f` by reference
    /// on each element.
    ///
    /// Elements are visited in arbitrary order.
    pub fn visit<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a A),
        A: 'a,
        S: Data,
    {
        self.fold((), move |(), elt| f(elt))
    }

    /// Fold along an axis.
    ///
    /// Combine the elements of each subview with the previous using the `fold`
    /// function and initial value `init`.
    ///
    /// Return the result as an `Array`.
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn fold_axis<B, F>(&self, axis: Axis, init: B, mut fold: F) -> Array<B, D::Smaller>
    where
        D: RemoveAxis,
        F: FnMut(&B, &A) -> B,
        B: Clone,
        S: Data,
    {
        let mut res = Array::from_elem(self.raw_dim().remove_axis(axis), init);
        for subview in self.axis_iter(axis) {
            res.zip_mut_with(&subview, |x, y| *x = fold(x, y));
        }
        res
    }

    /// Reduce the values along an axis into just one value, producing a new
    /// array with one less dimension.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return the result as an `Array`.
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn map_axis<'a, B, F>(&'a self, axis: Axis, mut mapping: F) -> Array<B, D::Smaller>
    where
        D: RemoveAxis,
        F: FnMut(ArrayView1<'a, A>) -> B,
        A: 'a,
        S: Data,
    {
        let view_len = self.len_of(axis);
        let view_stride = self.strides.axis(axis);
        if view_len == 0 {
            let new_dim = self.dim.remove_axis(axis);
            Array::from_shape_simple_fn(new_dim, move || mapping(ArrayView::from(&[])))
        } else {
            // use the 0th subview as a map to each 1d array view extended from
            // the 0th element.
            self.index_axis(axis, 0).map(|first_elt| unsafe {
                mapping(ArrayView::new_(first_elt, Ix1(view_len), Ix1(view_stride)))
            })
        }
    }

    /// Reduce the values along an axis into just one value, producing a new
    /// array with one less dimension.
    /// 1-dimensional lanes are passed as mutable references to the reducer,
    /// allowing for side-effects.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return the result as an `Array`.
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn map_axis_mut<'a, B, F>(&'a mut self, axis: Axis, mut mapping: F) -> Array<B, D::Smaller>
    where
        D: RemoveAxis,
        F: FnMut(ArrayViewMut1<'a, A>) -> B,
        A: 'a,
        S: DataMut,
    {
        let view_len = self.len_of(axis);
        let view_stride = self.strides.axis(axis);
        if view_len == 0 {
            let new_dim = self.dim.remove_axis(axis);
            Array::from_shape_simple_fn(new_dim, move || mapping(ArrayViewMut::from(&mut [])))
        } else {
            // use the 0th subview as a map to each 1d array view extended from
            // the 0th element.
            self.index_axis_mut(axis, 0).map_mut(|first_elt| unsafe {
                mapping(ArrayViewMut::new_(
                    first_elt,
                    Ix1(view_len),
                    Ix1(view_stride),
                ))
            })
        }
    }

    /// Iterates over pairs of consecutive elements along the axis.
    ///
    /// The first argument to the closure is an element, and the second
    /// argument is the next element along the axis. Iteration is guaranteed to
    /// proceed in order along the specified axis, but in all other respects
    /// the iteration order is unspecified.
    ///
    /// # Example
    ///
    /// For example, this can be used to compute the cumulative sum along an
    /// axis:
    ///
    /// ```
    /// use ndarray::{array, Axis};
    ///
    /// let mut arr = array![
    ///     [[1, 2], [3, 4], [5, 6]],
    ///     [[7, 8], [9, 10], [11, 12]],
    /// ];
    /// arr.accumulate_axis_inplace(Axis(1), |&prev, curr| *curr += prev);
    /// assert_eq!(
    ///     arr,
    ///     array![
    ///         [[1, 2], [4, 6], [9, 12]],
    ///         [[7, 8], [16, 18], [27, 30]],
    ///     ],
    /// );
    /// ```
    pub fn accumulate_axis_inplace<F>(&mut self, axis: Axis, mut f: F)
    where
        F: FnMut(&A, &mut A),
        S: DataMut,
    {
        if self.len_of(axis) <= 1 {
            return;
        }
        let mut curr = self.raw_view_mut(); // mut borrow of the array here
        let mut prev = curr.raw_view(); // derive further raw views from the same borrow
        prev.slice_axis_inplace(axis, Slice::from(..-1));
        curr.slice_axis_inplace(axis, Slice::from(1..));
        // This implementation relies on `Zip` iterating along `axis` in order.
        Zip::from(prev).and(curr).apply(|prev, curr| unsafe {
            // These pointer dereferences and borrows are safe because:
            //
            // 1. They're pointers to elements in the array.
            //
            // 2. `S: DataMut` guarantees that elements are safe to borrow
            //    mutably and that they don't alias.
            //
            // 3. The lifetimes of the borrows last only for the duration
            //    of the call to `f`, so aliasing across calls to `f`
            //    cannot occur.
            f(&*prev, &mut *curr)
        });
    }
}
