use std::ffi::{self, CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::str::{self, FromStr};

use super::{NulError, PdCStr, PdCString};

pub(crate) type PdCStringInner = CString;
pub(crate) type PdCStrInner = CStr;

#[doc(hidden)]
pub extern crate cstr;

#[macro_export]
/// A macro for creating a [`PdCStr`](crate::pdcstring::PdCStr) at compile time.
macro_rules! pdcstr {
    ($expression:expr) => {
        $crate::pdcstring::PdCStr::from_c_str($crate::pdcstring::cstr::cstr!($expression))
    };
}

// conversions to and from inner
impl PdCString {
    pub fn from_c_string(s: CString) -> Self {
        PdCString::from_inner(s)
    }
    pub fn into_c_string(self) -> CString {
        self.into_inner()
    }
}

// methods used by this crate
impl PdCString {
    pub fn from_os_str(s: impl AsRef<OsStr>) -> Result<Self, NulError> {
        PdCString::from_vec(s.as_ref().as_bytes().to_vec())
    }
    pub unsafe fn from_str_ptr(ptr: *const i8) -> Self {
        unsafe { PdCStr::from_str_ptr(ptr) }.to_owned()
    }
}

impl FromStr for PdCString {
    type Err = NulError;

    fn from_str(s: &str) -> Result<Self, NulError> {
        PdCString::from_vec(s.as_bytes().to_vec())
    }
}

// methods not used by this crate
impl PdCString {
    pub fn from_vec(vec: impl Into<Vec<u8>>) -> Result<Self, NulError> {
        let inner = CString::new(vec)?;
        Ok(PdCString::from_inner(inner))
    }
    pub fn into_vec(self) -> Vec<u8> {
        self.0.into_bytes()
    }
    pub fn into_vec_with_nul(self) -> Vec<u8> {
        self.0.into_bytes_with_nul()
    }
}

// conversions to and from inner
impl PdCStr {
    pub fn from_c_str(s: &CStr) -> &Self {
        PdCStr::from_inner(s)
    }
    pub fn to_c_str(&self) -> &CStr {
        self.to_inner()
    }
}

// methods used by this crate
impl PdCStr {
    pub fn as_ptr(&self) -> *const i8 {
        self.0.as_ptr()
    }
    pub unsafe fn from_str_ptr<'a>(ptr: *const i8) -> &'a Self {
        let inner = unsafe { CStr::from_ptr(ptr) };
        PdCStr::from_inner(inner)
    }
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[u8]) -> &Self {
        let inner = unsafe { CStr::from_bytes_with_nul_unchecked(slice) };
        PdCStr::from_inner(inner)
    }
    pub fn to_os_string(&self) -> OsString {
        self.to_os_str().to_owned()
    }
}

// methods not used by this crate
impl PdCStr {
    pub fn to_os_str(&self) -> &OsStr {
        OsStr::from_bytes(self.0.to_bytes())
    }
    // TODO: use abstract error type
    pub fn from_slice_with_nul(slice: &[u8]) -> Result<&Self, ffi::FromBytesWithNulError> {
        CStr::from_bytes_with_nul(slice).map(|s| PdCStr::from_inner(s))
    }
    pub fn to_slice(&self) -> &[u8] {
        self.0.to_bytes()
    }
    pub fn to_slice_with_nul(&self) -> &[u8] {
        self.0.to_bytes_with_nul()
    }
    pub fn len(&self) -> usize {
        self.0.to_bytes().len()
    }
    pub fn to_string(&self) -> Result<String, str::Utf8Error> {
        self.0.to_str().map(|s| s.to_string())
    }
    pub fn to_string_lossy(&self) -> String {
        self.0.to_string_lossy().to_string()
    }
}
