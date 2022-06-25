mod error;
pub use error::*;

/// The platform-dependent character type used by the hosting components.
pub type PdChar = crate::bindings::char_t;
/// The unsigned version of the platform-dependent character type used by the hosting components.
#[cfg(windows)]
pub type PdUChar = u16;
/// The unsigned version of the platform-dependent character type used by the hosting components.
#[cfg(not(windows))]
pub type PdUChar = u8;

mod r#impl;
pub use r#impl::*;

mod shared;
pub use shared::*;
