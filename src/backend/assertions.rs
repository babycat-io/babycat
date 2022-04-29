//! Custom assertions and other testing utilities.

use std::convert::AsRef;
use std::fmt::Debug;

/// Assert that an object's [`std::fmt::Debug`] output is equal to a specific value.
///
/// # Panics
/// This function is an assertion, therefore it might panic.
///
#[track_caller]
#[inline]
pub fn assert_debug<T: Debug, S: AsRef<str>>(t: &T, dbg_str: S) {
    assert_eq!(format!("{:?}", t), dbg_str.as_ref(),);
}
