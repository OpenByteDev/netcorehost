use std::ffi::OsString;

use widestring::U16CStr;

use crate::pdcstring::{MissingNulTerminator, PdCStrInner, PdChar, ToStringError};

#[doc(hidden)]
pub extern crate u16cstr;

#[macro_export]
/// A macro for creating a [`PdCStr`](crate::pdcstring::PdCStr) at compile time.
macro_rules! pdcstr {
    ($expression:expr) => {
        <$crate::pdcstring::PdCStr as $crate::pdcstring::windows::PdCStrExt>::from_u16_c_str(
            $crate::pdcstring::windows::u16cstr::u16cstr!($expression),
        )
    };
}

impl PdCStrInner for U16CStr {
    fn as_ptr(&self) -> *const PdChar {
        U16CStr::as_ptr(self)
    }
    unsafe fn from_str_ptr<'a>(ptr: *const PdChar) -> &'a Self {
        unsafe { U16CStr::from_ptr_str(ptr) }
    }
    unsafe fn from_slice_with_nul_unchecked(slice: &[PdChar]) -> &Self {
        unsafe { U16CStr::from_slice_unchecked(slice) }
    }
    fn to_os_string(&self) -> OsString {
        U16CStr::to_os_string(self)
    }

    fn from_slice_with_nul(slice: &[PdChar]) -> Result<&Self, MissingNulTerminator> {
        U16CStr::from_slice_truncate(slice).map_err(MissingNulTerminator)
    }

    fn to_slice(&self) -> &[PdChar] {
        U16CStr::as_slice(self)
    }

    fn to_slice_with_nul(&self) -> &[PdChar] {
        U16CStr::as_slice_with_nul(self)
    }

    fn is_empty(&self) -> bool {
        U16CStr::is_empty(self)
    }

    fn len(&self) -> usize {
        U16CStr::len(self)
    }

    fn to_string(&self) -> Result<String, ToStringError> {
        U16CStr::to_string(self).map_err(ToStringError)
    }

    fn to_string_lossy(&self) -> String {
        U16CStr::to_string_lossy(self)
    }
}
