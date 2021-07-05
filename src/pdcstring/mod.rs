mod error;
pub use error::*;

pub type PdChar = crate::bindings::char_t;
#[cfg(windows)]
pub type PdUChar = u16;
#[cfg(not(windows))]
pub type PdUChar = u8;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::*;

#[cfg(not(windows))]
mod other;
#[cfg(not(windows))]
use other::*;

mod shared;
pub use shared::*;
