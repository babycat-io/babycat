// Copyright 2014-2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::imp_prelude::*;
use crate::slice::MultiSlice;

/// Methods for read-only array views.
impl<'a, A, D> ArrayView<'a, A, D>
where
    D: Dimension,
{
    /// Split the array view along `axis` and return one view strictly before the
    /// split and one view after the split.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// **Examples:**
    /// ```rust
    /// # use ndarray::prelude::*;
    /// let a = aview2(&[[0, 1, 2, 3],
    ///                  [4, 5, 6, 7],
    ///                  [8, 9, 0, 1]]);
    ///
    /// ```
    /// The array view `a` has two axes and shape 3 × 4:
    /// ```text
    ///          ──▶ Axis(1)
    ///         ┌─────┬─────┬─────┬─────┐ 0
    ///       │ │ a₀₀ │ a₀₁ │ a₀₂ │ a₀₃ │
    ///       ▼ ├─────┼─────┼─────┼─────┤ 1
    ///  Axis(0)│ a₁₀ │ a₁₁ │ a₁₂ │ a₁₃ │
    ///         ├─────┼─────┼─────┼─────┤ 2
    ///         │ a₂₀ │ a₂₁ │ a₂₂ │ a₂₃ │
    ///         └─────┴─────┴─────┴─────┘ 3 ↑
    ///         0     1     2     3     4 ← possible split_at indices.
    /// ```
    ///
    /// Row indices increase along `Axis(0)`, and column indices increase along
    /// `Axis(1)`. Note that we split “before” an element index, and that
    /// both 0 and the endpoint are valid split indices.
    ///
    /// **Example 1**: Split `a` along the first axis, in this case the rows, at
    /// index 2.<br>
    /// This produces views v1 and v2 of shapes 2 × 4 and 1 × 4:
    ///
    /// ```rust
    /// # use ndarray::prelude::*;
    /// # let a = aview2(&[[0; 4]; 3]);
    /// let (v1, v2) = a.split_at(Axis(0), 2);
    /// ```
    /// ```text
    ///         ┌─────┬─────┬─────┬─────┐       0  ↓ indices
    ///         │ a₀₀ │ a₀₁ │ a₀₂ │ a₀₃ │            along Axis(0)
    ///         ├─────┼─────┼─────┼─────┤ v1    1
    ///         │ a₁₀ │ a₁₁ │ a₁₂ │ a₁₃ │
    ///         └─────┴─────┴─────┴─────┘
    ///         ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄       2
    ///         ┌─────┬─────┬─────┬─────┐
    ///         │ a₂₀ │ a₂₁ │ a₂₂ │ a₂₃ │ v2
    ///         └─────┴─────┴─────┴─────┘       3
    /// ```
    ///
    /// **Example 2**: Split `a` along the second axis, in this case the
    /// columns, at index 2.<br>
    /// This produces views u1 and u2 of shapes 3 × 2 and 3 × 2:
    ///
    /// ```rust
    /// # use ndarray::prelude::*;
    /// # let a = aview2(&[[0; 4]; 3]);
    /// let (u1, u2) = a.split_at(Axis(1), 2);
    ///
    /// ```
    /// ```text
    ///              u1             u2
    ///         ┌─────┬─────┐┊┌─────┬─────┐
    ///         │ a₀₀ │ a₀₁ │┊│ a₀₂ │ a₀₃ │
    ///         ├─────┼─────┤┊├─────┼─────┤
    ///         │ a₁₀ │ a₁₁ │┊│ a₁₂ │ a₁₃ │
    ///         ├─────┼─────┤┊├─────┼─────┤
    ///         │ a₂₀ │ a₂₁ │┊│ a₂₂ │ a₂₃ │
    ///         └─────┴─────┘┊└─────┴─────┘
    ///         0     1      2      3     4  indices →
    ///                                      along Axis(1)
    /// ```
    pub fn split_at(self, axis: Axis, index: Ix) -> (Self, Self) {
        unsafe {
            let (left, right) = self.into_raw_view().split_at(axis, index);
            (left.deref_into_view(), right.deref_into_view())
        }
    }
}

/// Methods for read-write array views.
impl<'a, A, D> ArrayViewMut<'a, A, D>
where
    D: Dimension,
{
    /// Split the array view along `axis` and return one mutable view strictly
    /// before the split and one mutable view after the split.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    pub fn split_at(self, axis: Axis, index: Ix) -> (Self, Self) {
        unsafe {
            let (left, right) = self.into_raw_view_mut().split_at(axis, index);
            (left.deref_into_view_mut(), right.deref_into_view_mut())
        }
    }

    /// Split the view into multiple disjoint slices.
    ///
    /// This is similar to [`.multi_slice_mut()`], but `.multi_slice_move()`
    /// consumes `self` and produces views with lifetimes matching that of
    /// `self`.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`SliceInfo`] and [`D::SliceArg`].
    ///
    /// [`.multi_slice_mut()`]: struct.ArrayBase.html#method.multi_slice_mut
    /// [`SliceInfo`]: struct.SliceInfo.html
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if any of the following occur:
    ///
    /// * if any of the views would intersect (i.e. if any element would appear in multiple slices)
    /// * if an index is out of bounds or step size is zero
    /// * if `D` is `IxDyn` and `info` does not match the number of array axes
    pub fn multi_slice_move<M>(self, info: M) -> M::Output
    where
        M: MultiSlice<'a, A, D>,
    {
        info.multi_slice_move(self)
    }
}
