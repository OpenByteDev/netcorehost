mod error;
pub use error::*;

pub type PdChar = crate::bindings::char_t;
#[cfg(windows)]
pub type PdUChar = u16;
#[cfg(not(windows))]
pub type PdUChar = u8;

mod r#impl;
pub use r#impl::*;

mod shared;
pub use shared::*;
