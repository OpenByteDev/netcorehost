use std::{
    ffi::{CStr, OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use crate::pdcstring::{MissingNulTerminator, PdCStrInner, PdChar, PdUChar, ToStringError};

#[doc(hidden)]
pub extern crate cstr;

#[macro_export]
/// A macro for creating a [`PdCStr`](crate::pdcstring::PdCStr) at compile time.
macro_rules! pdcstr {
    ($expression:expr) => {
        <$crate::pdcstring::PdCStr as $crate::pdcstring::other::PdCStrExt>::from_c_str(
            $crate::pdcstring::other::cstr::cstr!($expression),
        )
    };
}

impl PdCStrInner for CStr {
    fn as_ptr(&self) -> *const PdChar {
        CStr::as_ptr(self)
    }

    unsafe fn from_str_ptr<'a>(ptr: *const PdChar) -> &'a Self {
        unsafe { CStr::from_ptr(ptr) }
    }

    unsafe fn from_slice_with_nul_unchecked(slice: &[PdUChar]) -> &Self {
        unsafe { CStr::from_bytes_with_nul_unchecked(slice) }
    }

    fn to_os_string(&self) -> OsString {
        OsStr::from_bytes(CStr::to_bytes(self)).to_owned()
    }

    fn from_slice_with_nul(slice: &[PdUChar]) -> Result<&Self, MissingNulTerminator> {
        CStr::from_bytes_with_nul(slice).map_err(MissingNulTerminator)
    }

    fn to_slice(&self) -> &[PdUChar] {
        CStr::to_bytes(self)
    }

    fn to_slice_with_nul(&self) -> &[PdUChar] {
        CStr::to_bytes_with_nul(self)
    }

    fn is_empty(&self) -> bool {
        CStr::to_bytes(self).is_empty()
    }

    fn len(&self) -> usize {
        CStr::to_bytes(self).len()
    }

    fn to_string(&self) -> Result<String, ToStringError> {
        CStr::to_str(self)
            .map(str::to_string)
            .map_err(ToStringError)
    }

    fn to_string_lossy(&self) -> String {
        CStr::to_string_lossy(self).to_string()
    }
}
