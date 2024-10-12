#[cfg(not(any(qt_version_major = "5", qt_version_major = "6")))]
compile_error!("qt_version_major must be either \"5\" or \"6\"");

mod core;

pub use crate::core::*;
