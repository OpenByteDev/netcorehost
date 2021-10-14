use std::ffi::{OsStr, OsString};
use std::str::FromStr;
use std::string;
// use std::os::windows::ffi::OsStrExt;

use widestring::{U16CStr, U16CString};

use super::{NulError, PdCStr, PdCString};

pub(crate) type PdCStringInner = U16CString;
pub(crate) type PdCStrInner = U16CStr;

#[doc(hidden)]
pub extern crate u16cstr;

#[macro_export]
/// A macro for creating a [`PdCStr`](crate::pdcstring::PdCStr) at compile time.
macro_rules! pdcstr {
    ($expression:expr) => {
        $crate::pdcstring::PdCStr::from_u16_c_str($crate::pdcstring::u16cstr::u16cstr!($expression))
    };
}

// conversions to and from inner
impl PdCString {
    #[must_use]
    pub fn from_u16_c_string(s: U16CString) -> Self {
        PdCString::from_inner(s)
    }
    #[must_use]
    pub fn into_u16_c_string(self) -> U16CString {
        self.into_inner()
    }
}

// methods used by this crate
impl PdCString {
    pub fn from_os_str(s: impl AsRef<OsStr>) -> Result<Self, NulError> {
        let inner = U16CString::from_os_str(s)?;
        Ok(PdCString::from_u16_c_string(inner))
    }
    #[must_use]
    pub unsafe fn from_str_ptr(ptr: *const u16) -> Self {
        let inner = unsafe { U16CString::from_ptr_str(ptr) };
        PdCString::from_u16_c_string(inner)
    }
}

impl FromStr for PdCString {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, NulError> {
        let inner = U16CString::from_str(s)?;
        Ok(PdCString::from_u16_c_string(inner))
    }
}

// methods not used by this crate
impl PdCString {
    pub fn from_vec(vec: impl Into<Vec<u16>>) -> Result<Self, NulError> {
        let inner = U16CString::from_vec(vec)?;
        Ok(PdCString::from_inner(inner))
    }
    #[must_use]
    pub fn into_vec(self) -> Vec<u16> {
        self.0.into_vec()
    }
    #[must_use]
    pub fn into_vec_with_nul(self) -> Vec<u16> {
        self.0.into_vec_with_nul()
    }
}

// conversions to and from inner
impl PdCStr {
    #[must_use]
    pub fn from_u16_c_str(s: &U16CStr) -> &Self {
        PdCStr::from_inner(s)
    }
    #[must_use]
    pub fn to_u16_c_str(&self) -> &U16CStr {
        self.to_inner()
    }
}

// methods used by this crate
impl PdCStr {
    #[must_use]
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
    #[must_use]
    pub unsafe fn from_str_ptr<'a>(ptr: *const u16) -> &'a Self {
        let inner = unsafe { U16CStr::from_ptr_str(ptr) };
        PdCStr::from_inner(inner)
    }
    #[must_use]
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[u16]) -> &Self {
        let inner = unsafe { U16CStr::from_slice_unchecked(slice) };
        PdCStr::from_inner(inner)
    }
    #[must_use]
    pub fn to_os_string(&self) -> OsString {
        self.0.to_os_string()
    }
}

// methods not used by this crate
impl PdCStr {
    // TODO: use abstract error type
    pub fn from_slice_with_nul(
        slice: &[u16],
    ) -> Result<&Self, widestring::error::MissingNulTerminator> {
        U16CStr::from_slice_truncate(slice).map(PdCStr::from_inner)
    }
    #[must_use]
    pub fn to_slice(&self) -> &[u16] {
        self.0.as_slice()
    }
    #[must_use]
    pub fn to_slice_with_nul(&self) -> &[u16] {
        self.0.as_slice_with_nul()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn to_string(&self) -> Result<String, string::FromUtf16Error> {
        self.0.to_string()
    }
    #[must_use]
    pub fn to_string_lossy(&self) -> String {
        self.0.to_string_lossy()
    }
}
