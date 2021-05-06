/// Leak a str or String value.
/// This is useful for passing string values to an error enum.
macro_rules! leak_str {
    ($a:expr) => {
        Box::leak($a.to_owned().into_boxed_str())
    };
}

mod backend;

#[cfg(feature = "frontend-rust")]
pub use crate::backend::*;

pub mod frontends;
