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

#[cfg(windows)]
#[macro_export]
macro_rules! pdcstr {
    ($expression:expr) => {
        $crate::pdcstring::PdCStr::from_slice_with_nul(wchar::wchz!(u16, $expression)).unwrap()
    };
}

#[cfg(not(windows))]
#[macro_export]
macro_rules! pdcstr {
    ($expression:expr) => {
        $crate::pdcstring::PdCStr::from_c_str(cstr::cstr!($expression))
    };
}
