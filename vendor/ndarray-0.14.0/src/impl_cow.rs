// Copyright 2019 ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::imp_prelude::*;

/// Methods specific to `CowArray`.
///
/// ***See also all methods for [`ArrayBase`]***
///
/// [`ArrayBase`]: struct.ArrayBase.html
impl<'a, A, D> CowArray<'a, A, D>
where
    D: Dimension,
{
    /// Returns `true` iff the array is the view (borrowed) variant.
    pub fn is_view(&self) -> bool {
        self.data.is_view()
    }

    /// Returns `true` iff the array is the owned variant.
    pub fn is_owned(&self) -> bool {
        self.data.is_owned()
    }
}

impl<'a, A, D> From<ArrayView<'a, A, D>> for CowArray<'a, A, D>
where
    D: Dimension,
{
    fn from(view: ArrayView<'a, A, D>) -> CowArray<'a, A, D> {
        ArrayBase {
            data: CowRepr::View(view.data),
            ptr: view.ptr,
            dim: view.dim,
            strides: view.strides,
        }
    }
}

impl<'a, A, D> From<Array<A, D>> for CowArray<'a, A, D>
where
    D: Dimension,
{
    fn from(array: Array<A, D>) -> CowArray<'a, A, D> {
        ArrayBase {
            data: CowRepr::Owned(array.data),
            ptr: array.ptr,
            dim: array.dim,
            strides: array.strides,
        }
    }
}
